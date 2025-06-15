use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub tcp_addr: SocketAddr,
}

#[derive(Debug, Serialize, Deserialize)]
enum DiscoveryMessage {
    Announce(PeerInfo),
    Response(PeerInfo),
}

pub struct PeerNetwork {
    pub peer_id: String,
    tcp_listener: TcpListener,
    udp_socket: UdpSocket,
    peers: Arc<Mutex<HashMap<String, PeerInfo>>>,
    connections: Arc<Mutex<HashMap<String, TcpStream>>>,
    multicast_addr: SocketAddr,
}

impl PeerNetwork {
    pub fn new() -> io::Result<Self> {
        let peer_id = Uuid::new_v4().to_string();
        
        // Create TCP listener on any available port
        let tcp_listener = TcpListener::bind("0.0.0.0:0")?;
        let tcp_addr = tcp_listener.local_addr()?;
        
        // Create UDP socket for multicast discovery
        let udp_socket = UdpSocket::bind("0.0.0.0:8888")?;
        let multicast_addr: SocketAddr = "224.1.1.1:8888".parse().unwrap();
        
        // Join multicast group
        udp_socket.join_multicast_v4(
            &Ipv4Addr::new(224, 1, 1, 1),
            &Ipv4Addr::new(0, 0, 0, 0)
        )?;
        
        println!("Peer {} starting on TCP {}", peer_id, tcp_addr);
        
        Ok(PeerNetwork {
            peer_id,
            tcp_listener,
            udp_socket,
            peers: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            multicast_addr,
        })
    }
    
    pub fn start(&self) -> io::Result<()> {
        // Start TCP listener thread
        let tcp_listener = self.tcp_listener.try_clone()?;
        let peer_id = self.peer_id.clone();
        let connections = self.connections.clone();
        
        thread::spawn(move || {
            for stream in tcp_listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        // Simple handshake to get peer ID
                        let mut buffer = [0; 1024];
                        if let Ok(n) = stream.read(&mut buffer) {
                            if let Ok(remote_peer_id) = String::from_utf8(buffer[..n].to_vec()) {
                                println!("TCP connection from peer: {}", remote_peer_id);
                                
                                // Send our ID back
                                let _ = stream.write_all(peer_id.as_bytes());
                                
                                // Store connection
                                connections.lock().unwrap().insert(remote_peer_id.clone(), stream);
                            }
                        }
                    }
                    Err(e) => eprintln!("TCP accept error: {}", e),
                }
            }
        });
        
        // Start UDP discovery thread
        let udp_socket = self.udp_socket.try_clone()?;
        let peer_id = self.peer_id.clone();
        let tcp_addr = self.tcp_listener.local_addr()?;
        let peers = self.peers.clone();
        let connections = self.connections.clone();
        let multicast_addr = self.multicast_addr;
        
        thread::spawn(move || {
            let my_info = PeerInfo {
                id: peer_id.clone(),
                tcp_addr,
            };
            
            // Send initial announcement
            let announce_msg = DiscoveryMessage::Announce(my_info.clone());
            if let Ok(serialized) = bincode::serialize(&announce_msg) {
                let _ = udp_socket.send_to(&serialized, multicast_addr);
            }
            
            // Listen for discovery messages
            let mut buffer = [0; 1024];
            loop {
                match udp_socket.recv_from(&mut buffer) {
                    Ok((n, src)) => {
                        if let Ok(msg) = bincode::deserialize::<DiscoveryMessage>(&buffer[..n]) {
                            match msg {
                                DiscoveryMessage::Announce(peer_info) => {
                                    if peer_info.id != peer_id {
                                        println!("Discovered peer: {} at {}", peer_info.id, peer_info.tcp_addr);
                                        
                                        // Add to peer list
                                        peers.lock().unwrap().insert(peer_info.id.clone(), peer_info.clone());
                                        
                                        // Send response
                                        let response_msg = DiscoveryMessage::Response(my_info.clone());
                                        if let Ok(serialized) = bincode::serialize(&response_msg) {
                                            let _ = udp_socket.send_to(&serialized, multicast_addr);
                                        }
                                        
                                        // Establish TCP connection
                                        Self::connect_to_peer(&peer_info, &peer_id, &connections);
                                    }
                                }
                                DiscoveryMessage::Response(peer_info) => {
                                    if peer_info.id != peer_id {
                                        println!("Peer response from: {} at {}", peer_info.id, peer_info.tcp_addr);
                                        peers.lock().unwrap().insert(peer_info.id.clone(), peer_info.clone());
                                        Self::connect_to_peer(&peer_info, &peer_id, &connections);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("UDP receive error: {}", e),
                }
            }
        });
        
        // Send periodic announcements
        let udp_socket = self.udp_socket.try_clone()?;
        let peer_id = self.peer_id.clone();
        let tcp_addr = self.tcp_listener.local_addr()?;
        let multicast_addr = self.multicast_addr;
        
        thread::spawn(move || {
            let my_info = PeerInfo {
                id: peer_id,
                tcp_addr,
            };
            
            loop {
                thread::sleep(Duration::from_secs(5));
                let announce_msg = DiscoveryMessage::Announce(my_info.clone());
                if let Ok(serialized) = bincode::serialize(&announce_msg) {
                    let _ = udp_socket.send_to(&serialized, multicast_addr);
                }
            }
        });
        
        Ok(())
    }
    
    fn connect_to_peer(
        peer_info: &PeerInfo,
        my_peer_id: &str,
        connections: &Arc<Mutex<HashMap<String, TcpStream>>>,
    ) {
        // Avoid duplicate connections by only connecting to peers with higher IDs
        if peer_info.id <= *my_peer_id {
            return;
        }
        
        // Check if already connected
        if connections.lock().unwrap().contains_key(&peer_info.id) {
            return;
        }
        
        // Attempt TCP connection
        match TcpStream::connect(peer_info.tcp_addr) {
            Ok(mut stream) => {
                // Send handshake
                if stream.write_all(my_peer_id.as_bytes()).is_ok() {
                    // Read response
                    let mut buffer = [0; 1024];
                    if let Ok(n) = stream.read(&mut buffer) {
                        if let Ok(remote_id) = String::from_utf8(buffer[..n].to_vec()) {
                            if remote_id == peer_info.id {
                                println!("TCP connected to peer: {}", peer_info.id);
                                connections.lock().unwrap().insert(peer_info.id.clone(), stream);
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to connect to {}: {}", peer_info.id, e),
        }
    }
    
    pub fn send_to_peer(&self, peer_id: &str, data: &[u8]) -> io::Result<()> {
        let mut connections = self.connections.lock().unwrap();
        if let Some(stream) = connections.get_mut(peer_id) {
            stream.write_all(data)?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Peer not connected"))
        }
    }
    
    pub fn broadcast(&self, data: &[u8]) -> io::Result<()> {
        let mut connections = self.connections.lock().unwrap();
        for (peer_id, stream) in connections.iter_mut() {
            if let Err(e) = stream.write_all(data) {
                eprintln!("Failed to send to peer {}: {}", peer_id, e);
            }
        }
        Ok(())
    }
    
    pub fn get_connected_peers(&self) -> Vec<String> {
        self.connections.lock().unwrap().keys().cloned().collect()
    }
    
    pub fn get_peer_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
}

// Example usage
pub fn run_example() -> io::Result<()> {
    let network = PeerNetwork::new()?;
    network.start()?;
    
    println!("Network started. Press Enter to send test messages...");
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let connected_peers = network.get_connected_peers();
        println!("Connected to {} peers: {:?}", connected_peers.len(), connected_peers);
        
        // Send test message to all peers
        let message = format!("Hello from peer {}", network.peer_id);
        network.broadcast(message.as_bytes())?;
    }
}