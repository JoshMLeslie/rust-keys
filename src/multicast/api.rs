// api.rs - Improved implementation with Result-like broadcast results
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind};
use super::multicast::PeerNetwork;
use super::types::{PeerInfo, PeerMessage};

pub trait Api {
    fn send_to_peer(&self, peer_id: &str, data: &[u8]) -> io::Result<()>;
    fn broadcast(&self, data: &[u8]) -> io::Result<usize>;
    fn get_connected_peers(&self) -> Vec<String>;
    fn get_peer_count(&self) -> usize;
    fn is_connected_to(&self, peer_id: &str) -> bool;
    fn disconnect_peer(&self, peer_id: &str) -> io::Result<()>;
    fn get_peer_info(&self, peer_id: &str) -> Option<PeerInfo>;
}

#[derive(Debug)]
pub struct BroadcastResult {
    pub results: HashMap<String, Result<(), String>>, // peer_id -> Result
}

impl BroadcastResult {
    pub fn successful_count(&self) -> usize {
        self.results.values().filter(|r| r.is_ok()).count()
    }
    
    pub fn failed_count(&self) -> usize {
        self.results.values().filter(|r| r.is_err()).count()
    }
    
    pub fn successful_peers(&self) -> Vec<&String> {
        self.results.iter()
            .filter(|(_, result)| result.is_ok())
            .map(|(peer_id, _)| peer_id)
            .collect()
    }
    
    pub fn failed_peers(&self) -> Vec<(&String, &String)> {
        self.results.iter()
            .filter_map(|(peer_id, result)| {
                if let Err(error) = result {
                    Some((peer_id, error))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn is_complete_success(&self) -> bool {
        self.results.values().all(|r| r.is_ok())
    }

    pub fn is_complete_failure(&self) -> bool {
        self.results.values().all(|r| r.is_err())
    }
}

impl Api for PeerNetwork {
    /// Send data to a specific peer
    fn send_to_peer(&self, peer_id: &str, data: &[u8]) -> io::Result<()> {
        if data.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Cannot send empty data"));
        }

        if data.len() > 1024 * 1024 {  // 1MB limit
            return Err(Error::new(ErrorKind::InvalidInput, "Message too large"));
        }

        let message = PeerMessage::Data(data.to_vec());
        
        let mut connections = self.get_connections().lock()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to acquire connection lock"))?;
        
        if let Some(connection) = connections.get_mut(peer_id) {
            connection.send_message(&message)
                .map_err(|e| {
                    eprintln!("Failed to send to peer {}: {}", peer_id, e);
                    e
                })
        } else {
            Err(Error::new(ErrorKind::NotConnected, format!("Peer '{}' not connected", peer_id)))
        }
    }

    /// Broadcast data to all connected peers
    /// Returns the number of successful sends
    fn broadcast(&self, data: &[u8]) -> io::Result<usize> {
        let result = self.broadcast_detailed(data)?;
        Ok(result.successful_count())
    }

    /// Get list of currently connected peer IDs
    fn get_connected_peers(&self) -> Vec<String> {
        self.get_connections().lock()
            .map(|connections| connections.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Get count of currently connected peers
    fn get_peer_count(&self) -> usize {
        self.get_connections().lock()
            .map(|connections| connections.len())
            .unwrap_or(0)
    }

    /// Check if connected to a specific peer
    fn is_connected_to(&self, peer_id: &str) -> bool {
        self.get_connections().lock()
            .map(|connections| connections.contains_key(peer_id))
            .unwrap_or(false)
    }

    /// Gracefully disconnect from a specific peer
    fn disconnect_peer(&self, peer_id: &str) -> io::Result<()> {
        let mut connections = self.get_connections().lock()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to acquire connection lock"))?;
        
        if let Some(mut connection) = connections.remove(peer_id) {
            // Send disconnect message
            let _ = connection.send_message(&PeerMessage::Disconnect);
            println!("Disconnected from peer: {}", peer_id);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotConnected, format!("Peer '{}' not connected", peer_id)))
        }
    }

    /// Get information about a discovered peer (may not be connected)
    fn get_peer_info(&self, peer_id: &str) -> Option<PeerInfo> {
        self.get_peers().read()
            .ok()
            .and_then(|peers| peers.get(peer_id).cloned())
    }
}

// Extended API for more detailed broadcast results
impl PeerNetwork {
    /// Broadcast with detailed results about success/failure per peer
    pub fn broadcast_detailed(&self, data: &[u8]) -> io::Result<BroadcastResult> {
        if data.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Cannot broadcast empty data"));
        }

        if data.len() > 1024 * 1024 {  // 1MB limit
            return Err(Error::new(ErrorKind::InvalidInput, "Message too large"));
        }

        let message = PeerMessage::Data(data.to_vec());
        let mut results = HashMap::new();
        let mut failed_peer_ids = Vec::new();

        {
            let mut connections = self.get_connections().lock()
                .map_err(|_| Error::new(ErrorKind::Other, "Failed to acquire connection lock"))?;
            
            for (peer_id, connection) in connections.iter_mut() {
                let result = match connection.send_message(&message) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("Failed to send to peer {}: {}", peer_id, e);
                        failed_peer_ids.push(peer_id.clone());
                        Err(e.to_string())
                    }
                };
                results.insert(peer_id.clone(), result);
            }

            // Remove failed connections
            for peer_id in failed_peer_ids {
                connections.remove(&peer_id);
                println!("Removed failed connection to peer: {}", peer_id);
            }
        }

        Ok(BroadcastResult { results })
    }

    /// Get all discovered peers (both connected and disconnected)
    pub fn get_all_discovered_peers(&self) -> Vec<PeerInfo> {
        self.get_peers().read()
            .map(|peers| peers.values().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        let connected_count = self.get_peer_count();
        let discovered_count = self.get_peers().read()
            .map(|peers| peers.len())
            .unwrap_or(0);

        NetworkStats {
            connected_peers: connected_count,
            discovered_peers: discovered_count,
            peer_id: self.peer_id.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub connected_peers: usize,
    pub discovered_peers: usize,
    pub peer_id: String,
}

// Convenience functions for common operations
impl PeerNetwork {
    /// Send a text message to a specific peer
    pub fn send_text_to_peer(&self, peer_id: &str, message: &str) -> io::Result<()> {
        self.send_to_peer(peer_id, message.as_bytes())
    }

    /// Broadcast a text message to all connected peers
    pub fn broadcast_text(&self, message: &str) -> io::Result<usize> {
        self.broadcast(message.as_bytes())
    }

    /// Broadcast a text message with detailed results
    pub fn broadcast_text_detailed(&self, message: &str) -> io::Result<BroadcastResult> {
        self.broadcast_detailed(message.as_bytes())
    }

    /// Check if the network has any connected peers
    pub fn has_peers(&self) -> bool {
        self.get_peer_count() > 0
    }

    /// Wait for at least one peer to connect (with timeout)
    pub fn wait_for_peers(&self, timeout: std::time::Duration) -> bool {
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout {
            if self.has_peers() {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_result_methods() {
        let mut results = HashMap::new();
        results.insert("peer1".to_string(), Ok(()));
        results.insert("peer2".to_string(), Err("Connection failed".to_string()));
        results.insert("peer3".to_string(), Ok(()));

        let broadcast_result = BroadcastResult { results };

        assert_eq!(broadcast_result.successful_count(), 2);
        assert_eq!(broadcast_result.failed_count(), 1);
        assert!(!broadcast_result.is_complete_success());
        assert!(!broadcast_result.is_complete_failure());

        let successful_peers = broadcast_result.successful_peers();
        assert_eq!(successful_peers.len(), 2);
        assert!(successful_peers.contains(&&"peer1".to_string()));
        assert!(successful_peers.contains(&&"peer3".to_string()));

        let failed_peers = broadcast_result.failed_peers();
        assert_eq!(failed_peers.len(), 1);
        assert_eq!(failed_peers[0].0, &"peer2".to_string());
        assert_eq!(failed_peers[0].1, &"Connection failed".to_string());
    }
}

// Usage examples:
/*
fn usage_examples() -> io::Result<()> {
    let network = PeerNetwork::new("my_peer")?;
    
    // Simple broadcast (just get count)
    let success_count = network.broadcast(b"hello world")?;
    println!("Successfully sent to {} peers", success_count);
    
    // Detailed broadcast with per-peer results
    let result = network.broadcast_detailed(b"hello world")?;
    
    // Check overall success
    if result.is_complete_success() {
        println!("Message sent to all {} peers successfully!", result.successful_count());
    } else {
        println!("Sent to {}/{} peers", 
                 result.successful_count(), 
                 result.results.len());
    }
    
    // Handle individual failures
    for (peer_id, error) in result.failed_peers() {
        println!("Failed to send to {}: {}", peer_id, error);
    }
    
    // Check specific peer result
    if let Some(peer_result) = result.results.get("specific_peer") {
        match peer_result {
            Ok(_) => println!("Successfully sent to specific_peer"),
            Err(error) => println!("Failed to send to specific_peer: {}", error),
        }
    }
    
    Ok(())
}
*/