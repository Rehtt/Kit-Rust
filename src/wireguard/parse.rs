use crate::interface::{Interface, Peer};

impl Interface {
    pub fn parse_wg(data: String) -> Result<Interface, bool> {
        let mut  a = Interface {
            name: String::from("123"),
            public_key: "".to_string(),
            listen_port: 0,
            peers: Vec::new(),
        };
        a.peers.push(Peer{ public_key: "".to_string(), endpoint: "".to_string(), latest_handshake: Default::default() });

        Ok(a)
    }
}