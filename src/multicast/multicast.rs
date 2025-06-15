use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::multicast::types::{Connection, DiscoveryMessage, PeerInfo, PeerMessage};

const MULTICAST_ADDR: &str = "239.255.42.1:8888";
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(5);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer_stream = stream.try_clone()?;
        Ok(Connection {
            stream,
            writer: BufWriter::new(writer_stream),
            last_activity: Instant::now(),
        })
    }

    pub fn send_message(&mut self, message: &PeerMessage) -> io::Result<()> {
        let serialized = bincode::encode_to_vec(message, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Write length prefix (4 bytes, big-endian)
        let len = serialized.len() as u32;
        self.writer.write_all(&len.to_be_bytes())?;
        self.writer.write_all(&serialized)?;
        self.writer.flush()?;

        self.last_activity = Instant::now();
        Ok(())
    }

    pub fn is_stale(&self) -> bool {
        self.last_activity.elapsed() > HEARTBEAT_INTERVAL * 2
    }
}

pub struct PeerNetwork {
    pub peer_id: String,
    tcp_listener: TcpListener,
    udp_socket: UdpSocket,
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    connections: Arc<Mutex<HashMap<String, Connection>>>,
    multicast_addr: SocketAddr,
    shutdown: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl PeerNetwork {
    pub fn new() -> io::Result<Self> {
        let peer_id = Uuid::new_v4().to_string();

        // Create TCP listener on any available port
        let tcp_listener = TcpListener::bind("0.0.0.0:0")?;
        let tcp_addr = tcp_listener.local_addr()?;

        // Create UDP socket for multicast discovery
        let udp_socket = UdpSocket::bind("0.0.0.0:8888")?;
        let multicast_addr: SocketAddr = MULTICAST_ADDR
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        // Join multicast group
        udp_socket
            .join_multicast_v4(&Ipv4Addr::new(239, 255, 42, 1), &Ipv4Addr::new(0, 0, 0, 0))?;

        // Set socket timeouts
        udp_socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        println!("Peer {} starting on TCP {}", peer_id, tcp_addr);

        Ok(PeerNetwork {
            peer_id,
            tcp_listener,
            udp_socket,
            peers: Arc::new(RwLock::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            multicast_addr,
            shutdown: Arc::new(AtomicBool::new(false)),
            threads: Vec::new(),
        })
    }

    pub fn start(&mut self) -> io::Result<()> {
        // Start TCP listener thread
        let tcp_listener = self.tcp_listener.try_clone()?;
        let peer_id = self.peer_id.clone();
        let connections = self.connections.clone();
        let shutdown = self.shutdown.clone();

        let tcp_handle = thread::spawn(move || {
            Self::tcp_listener_thread(tcp_listener, peer_id, connections, shutdown);
        });

        // Start UDP discovery thread
        let udp_socket = self.udp_socket.try_clone()?;
        let peer_id = self.peer_id.clone();
        let tcp_addr = self.tcp_listener.local_addr()?;
        let peers = self.peers.clone();
        let connections = self.connections.clone();
        let multicast_addr = self.multicast_addr;
        let shutdown = self.shutdown.clone();

        let udp_handle = thread::spawn(move || {
            Self::udp_discovery_thread(
                udp_socket,
                peer_id,
                tcp_addr,
                peers,
                connections,
                multicast_addr,
                shutdown,
            );
        });

        // Start connection maintenance thread
        let connections = self.connections.clone();
        let shutdown = self.shutdown.clone();

        let maintenance_handle = thread::spawn(move || {
            Self::connection_maintenance_thread(connections, shutdown);
        });

        self.threads = vec![tcp_handle, udp_handle, maintenance_handle];
        Ok(())
    }

    pub fn shutdown(&self) {
        println!("Shutting down peer network...");
        self.shutdown.store(true, Ordering::Relaxed);

        // Send disconnect messages to all peers
        if let Ok(mut connections) = self.connections.lock() {
            for (peer_id, connection) in connections.iter_mut() {
                if let Err(e) = connection.send_message(&PeerMessage::Disconnect) {
                    eprintln!("Failed to send disconnect to {}: {}", peer_id, e);
                }
            }
        }
    }

    fn tcp_listener_thread(
        tcp_listener: TcpListener,
        peer_id: String,
        connections: Arc<Mutex<HashMap<String, Connection>>>,
        shutdown: Arc<AtomicBool>,
    ) {
        // Set non-blocking to check for shutdown
        if let Err(e) = tcp_listener.set_nonblocking(true) {
            eprintln!("Failed to set TCP listener non-blocking: {}", e);
            return;
        }

        while !shutdown.load(Ordering::Relaxed) {
            match tcp_listener.accept() {
                Ok((stream, _)) => {
                    if let Err(e) = Self::handle_incoming_connection(stream, &peer_id, &connections)
                    {
                        eprintln!("Failed to handle incoming connection: {}", e);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    eprintln!("TCP accept error: {}", e);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    fn handle_incoming_connection(
        stream: TcpStream,
        peer_id: &str,
        connections: &Arc<Mutex<HashMap<String, Connection>>>,
    ) -> io::Result<()> {
        // Set connection timeout
        stream.set_read_timeout(Some(CONNECTION_TIMEOUT))?;
        stream.set_write_timeout(Some(CONNECTION_TIMEOUT))?;

        // Perform handshake
        let mut reader = BufReader::new(&stream);
        let mut remote_peer_id = String::new();
        reader.read_line(&mut remote_peer_id)?;
        let remote_peer_id = remote_peer_id.trim().to_string();

        if remote_peer_id.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty peer ID received",
            ));
        }

        // Send our ID back
        writeln!(&stream, "{}", peer_id)?;

        println!("TCP connection established with peer: {}", remote_peer_id);

        // Store connection
        let connection = Connection::new(stream)?;
        connections
            .lock()
            .unwrap()
            .insert(remote_peer_id.clone(), connection);

        // Start message handling for this connection
        Self::start_connection_handler(remote_peer_id, connections.clone());

        Ok(())
    }

    fn start_connection_handler(
        peer_id: String,
        connections: Arc<Mutex<HashMap<String, Connection>>>,
    ) {
        thread::spawn(move || {
            let mut buffer = [0u8; 4];

            loop {
                // Get the stream (need to do this each iteration due to borrowing)
                let stream = {
                    let connections_guard = connections.lock().unwrap();
                    if let Some(connection) = connections_guard.get(&peer_id) {
                        connection.stream.try_clone()
                    } else {
                        break; // Connection removed
                    }
                };

                let mut stream = match stream {
                    Ok(s) => s,
                    Err(_) => break,
                };

                // Set non-blocking for reading
                if stream.set_nonblocking(true).is_err() {
                    break;
                }

                // Try to read message length
                match stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        let message_len = u32::from_be_bytes(buffer) as usize;
                        if message_len > 1024 * 1024 {
                            // 1MB limit
                            eprintln!("Message too large from {}: {} bytes", peer_id, message_len);
                            break;
                        }

                        let mut message_buffer = vec![0u8; message_len];
                        if stream.read_exact(&mut message_buffer).is_ok() {
                            if let Ok((message, _)) = bincode::decode_from_slice::<PeerMessage, _>(
                                &message_buffer,
                                bincode::config::standard(),
                            ) {
                                match message {
                                    PeerMessage::Data(data) => {
                                        if let Ok(text) = String::from_utf8(data) {
                                            println!("Message from {}: {}", peer_id, text.trim());
                                        }
                                    }
                                    PeerMessage::Heartbeat => {
                                        // Update last activity
                                        if let Ok(mut connections_guard) = connections.lock() {
                                            if let Some(connection) =
                                                connections_guard.get_mut(&peer_id)
                                            {
                                                connection.last_activity = Instant::now();
                                            }
                                        }
                                    }
                                    PeerMessage::Disconnect => {
                                        println!("Peer {} disconnected gracefully", peer_id);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(_) => break,
                }
            }

            // Remove connection when done
            connections.lock().unwrap().remove(&peer_id);
            println!("Connection handler for {} terminated", peer_id);
        });
    }

    fn udp_discovery_thread(
        udp_socket: UdpSocket,
        peer_id: String,
        tcp_addr: SocketAddr,
        peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
        connections: Arc<Mutex<HashMap<String, Connection>>>,
        multicast_addr: SocketAddr,
        shutdown: Arc<AtomicBool>,
    ) {
        let my_info = PeerInfo {
            id: peer_id.clone(),
            tcp_addr,
            last_seen: Self::current_timestamp(),
        };

        // Send initial announcement
        Self::send_discovery_message(
            &udp_socket,
            &DiscoveryMessage::Announce(my_info.clone()),
            multicast_addr,
        );

        let mut buffer = [0; 1024];
        let mut last_announce = Instant::now();

        while !shutdown.load(Ordering::Relaxed) {
            // Send periodic announcements
            if last_announce.elapsed() >= DISCOVERY_INTERVAL {
                Self::send_discovery_message(
                    &udp_socket,
                    &DiscoveryMessage::Announce(my_info.clone()),
                    multicast_addr,
                );
                last_announce = Instant::now();
            }

            // Listen for discovery messages
            match udp_socket.recv_from(&mut buffer) {
                Ok((n, _src)) => {
                    if let Ok((msg, _)) = bincode::decode_from_slice::<DiscoveryMessage, _>(
                        &buffer[..n],
                        bincode::config::standard(),
                    ) {
                        Self::handle_discovery_message(
                            msg,
                            &peer_id,
                            &my_info,
                            &peers,
                            &connections,
                            &udp_socket,
                            multicast_addr,
                        );
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    // Timeout is expected, continue
                }
                Err(e) => {
                    eprintln!("UDP receive error: {}", e);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    fn handle_discovery_message(
        msg: DiscoveryMessage,
        my_peer_id: &str,
        my_info: &PeerInfo,
        peers: &Arc<RwLock<HashMap<String, PeerInfo>>>,
        connections: &Arc<Mutex<HashMap<String, Connection>>>,
        udp_socket: &UdpSocket,
        multicast_addr: SocketAddr,
    ) {
        match msg {
            DiscoveryMessage::Announce(peer_info) => {
                if peer_info.id != my_peer_id {
                    println!(
                        "Discovered peer: {} at {}",
                        peer_info.id, peer_info.tcp_addr
                    );

                    // Add to peer list
                    peers
                        .write()
                        .unwrap()
                        .insert(peer_info.id.clone(), peer_info.clone());

                    // Send response
                    Self::send_discovery_message(
                        udp_socket,
                        &DiscoveryMessage::Response(my_info.clone()),
                        multicast_addr,
                    );

                    // Establish TCP connection
                    Self::connect_to_peer(&peer_info, my_peer_id, connections);
                }
            }
            DiscoveryMessage::Response(peer_info) => {
                if peer_info.id != my_peer_id {
                    println!(
                        "Peer response from: {} at {}",
                        peer_info.id, peer_info.tcp_addr
                    );
                    peers
                        .write()
                        .unwrap()
                        .insert(peer_info.id.clone(), peer_info.clone());
                    Self::connect_to_peer(&peer_info, my_peer_id, connections);
                }
            }
            DiscoveryMessage::Heartbeat(peer_id) => {
                if let Ok(mut peers_guard) = peers.write() {
                    if let Some(peer_info) = peers_guard.get_mut(&peer_id) {
                        peer_info.last_seen = Self::current_timestamp();
                    }
                }
            }
        }
    }

    fn send_discovery_message(
        udp_socket: &UdpSocket,
        message: &DiscoveryMessage,
        addr: SocketAddr,
    ) {
        if let Ok(serialized) = bincode::encode_to_vec(message, bincode::config::standard()) {
            if let Err(e) = udp_socket.send_to(&serialized, addr) {
                eprintln!("Failed to send discovery message: {}", e);
            }
        }
    }

    fn connect_to_peer(
        peer_info: &PeerInfo,
        my_peer_id: &str,
        connections: &Arc<Mutex<HashMap<String, Connection>>>,
    ) {
        // Avoid duplicate connections by only connecting to peers with higher IDs
        if peer_info.id.as_str() <= my_peer_id {
            return;
        }

        // Check if already connected
        if connections.lock().unwrap().contains_key(&peer_info.id) {
            return;
        }

        let peer_id = peer_info.id.clone();
        let tcp_addr = peer_info.tcp_addr;
        let my_peer_id = my_peer_id.to_string();
        let connections = connections.clone();

        // Connect in a separate thread to avoid blocking
        thread::spawn(move || {
            match TcpStream::connect_timeout(&tcp_addr, CONNECTION_TIMEOUT) {
                Ok(mut stream) => {
                    // Send handshake
                    if writeln!(stream, "{}", my_peer_id).is_ok() {
                        // Read response
                        let mut reader = BufReader::new(&stream);
                        let mut response = String::new();
                        if reader.read_line(&mut response).is_ok() {
                            let response = response.trim();
                            if response == peer_id {
                                println!("TCP connected to peer: {}", peer_id);

                                if let Ok(connection) = Connection::new(stream) {
                                    connections
                                        .lock()
                                        .unwrap()
                                        .insert(peer_id.clone(), connection);
                                    Self::start_connection_handler(peer_id, connections);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to connect to {}: {}", peer_id, e);
                }
            }
        });
    }

    fn connection_maintenance_thread(
        connections: Arc<Mutex<HashMap<String, Connection>>>,
        shutdown: Arc<AtomicBool>,
    ) {
        while !shutdown.load(Ordering::Relaxed) {
            thread::sleep(HEARTBEAT_INTERVAL);

            let mut to_remove = Vec::new();

            // Send heartbeats and check for stale connections
            if let Ok(mut connections_guard) = connections.lock() {
                for (peer_id, connection) in connections_guard.iter_mut() {
                    if connection.is_stale() {
                        println!("Connection to {} is stale, removing", peer_id);
                        to_remove.push(peer_id.clone());
                    } else {
                        // Send heartbeat
                        if let Err(e) = connection.send_message(&PeerMessage::Heartbeat) {
                            eprintln!("Failed to send heartbeat to {}: {}", peer_id, e);
                            to_remove.push(peer_id.clone());
                        }
                    }
                }

                // Remove stale connections
                for peer_id in to_remove {
                    connections_guard.remove(&peer_id);
                }
            }
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl PeerNetwork {
    // Getter methods for API access
    pub fn get_connections(&self) -> &Arc<Mutex<HashMap<String, Connection>>> {
        &self.connections
    }

    pub fn get_peers(&self) -> &Arc<RwLock<HashMap<String, PeerInfo>>> {
        &self.peers
    }
}

impl Drop for PeerNetwork {
    fn drop(&mut self) {
        self.shutdown();

        // Wait for threads to finish (with timeout)
        let threads = std::mem::take(&mut self.threads);
        for handle in threads {
            let _ = handle.join();
        }
    }
}
