use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::net::TcpStream;

use crate::peer_network::{PeerNetwork, Api};

pub fn run_interactive_example() -> io::Result<()> {
    let network = PeerNetwork::new()?;
    network.start()?;
    
    println!("Network started. The network runs in background threads.");
    println!("Commands: 'peers' to list connected peers, 'send <message>' to broadcast, 'quit' to exit");
    
    // Spawn a thread to handle incoming messages from peers
    let connections = network.connections.clone();
    let peer_id = network.peer_id.clone();
    
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            
            // Check for incoming data from all connected peers
            let mut connections_guard = connections.lock().unwrap();
            let mut to_remove = Vec::new();
            
            for (peer_id, stream) in connections_guard.iter_mut() {
                // Set non-blocking mode to avoid hanging
                if stream.set_nonblocking(true).is_ok() {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            // Connection closed
                            println!("Peer {} disconnected", peer_id);
                            to_remove.push(peer_id.clone());
                        }
                        Ok(n) => {
                            // Received data
                            if let Ok(message) = String::from_utf8(buffer[..n].to_vec()) {
                                println!("Message from {}: {}", peer_id, message.trim());
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // No data available, continue
                        }
                        Err(e) => {
                            println!("Error reading from peer {}: {}", peer_id, e);
                            to_remove.push(peer_id.clone());
                        }
                    }
                    // Reset to blocking mode
                    let _ = stream.set_nonblocking(false);
                }
            }
            
            // Remove disconnected peers
            for peer_id in to_remove {
                connections_guard.remove(&peer_id);
            }
        }
    });
    
    // Main thread handles user input (non-blocking for the network)
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                match input {
                    "quit" | "exit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "peers" => {
                        let connected_peers = network.get_connected_peers();
                        println!("Connected to {} peers: {:?}", connected_peers.len(), connected_peers);
                    }
                    input if input.starts_with("send ") => {
                        let message = &input[5..]; // Skip "send "
                        let full_message = format!("[{}]: {}", network.peer_id, message);
                        match network.broadcast(full_message.as_bytes()) {
                            Ok(_) => println!("Message broadcasted to {} peers", network.get_peer_count()),
                            Err(e) => println!("Broadcast failed: {}", e),
                        }
                    }
                    _ => {
                        println!("Unknown command: {}", input);
                        println!("Available commands: 'peers', 'send <message>', 'quit'");
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

pub fn run_simple_example() -> io::Result<()> {
    let network = PeerNetwork::new()?;
    network.start()?;
    
    println!("Simple network example running...");
    println!("Peer ID: {}", network.peer_id);
    
    // Simple loop that shows peer count every 3 seconds
    loop {
        thread::sleep(Duration::from_secs(3));
        let peer_count = network.get_peer_count();
        let peers = network.get_connected_peers();
        
        println!("Connected to {} peers: {:?}", peer_count, peers);
        
        if peer_count > 0 {
            let test_message = format!("Hello from peer {}", network.peer_id);
            if let Err(e) = network.broadcast(test_message.as_bytes()) {
                println!("Broadcast failed: {}", e);
            }
        }
    }
}