// multicast.rs - Improved implementation
use bincode::{Decode, Encode};
use std::io::BufWriter;
use std::net::{SocketAddr, TcpStream};
use std::time::Instant;

#[derive(Debug, Clone, Encode, Decode)]
pub struct PeerInfo {
    pub id: String,
    pub tcp_addr: SocketAddr,
    pub last_seen: u64, // Unix timestamp
}

#[derive(Debug, Encode, Decode)]
pub enum DiscoveryMessage {
    Announce(PeerInfo),
    Response(PeerInfo),
    Heartbeat(String), // peer_id
}

#[derive(Debug, Encode, Decode)]
pub enum PeerMessage {
    Data(Vec<u8>),
    Heartbeat,
    Disconnect,
}

pub struct Connection {
    pub stream: TcpStream,
    pub writer: BufWriter<TcpStream>,
    pub last_activity: Instant,
}
