use std::net::IpAddr;
use std::time::Duration;
use crate::ByteSize;

pub struct Interface {
    pub name: String,
    pub public_key: String,
    pub listen_port: u16,
    pub peers: Vec<Peer>,
}

pub struct Peer {
    pub public_key: String,
    pub endpoint: String,
    pub allowed: Vec<IpAddr::partial_cmp()>,
    pub latest_handshake: Duration,
    pub transfer: Transfer,
    pub keepalive: Duration,
}

pub struct Transfer {
    pub received: ByteSize,
    pub sent: ByteSize,
}