extern crate rand;
extern crate serde;

use crate::consensus::config::Config;
use crate::consensus::node::{Node, State};
use crate::consensus::message::Message;
use crate::consensus::message::Message::{Ack, Join, Ping, SendPeers};

use rand::{thread_rng, Rng};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread::{Builder, sleep, JoinHandle};


pub struct Peers {
    config: Config,
    /// A map where the key is the address <ip>:<port> and the value is a Node.
    nodes: RwLock<BTreeMap<String, Node>>,
}

impl Peers {
    pub fn with_config(config: Config) -> Peers {
        Peers {
            config: config,
            nodes: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn start(peers: Arc<Self>) -> Result<Vec<JoinHandle<()>>, ()> {
        let self1 = peers.clone();
        let self2 = self1.clone();

        let ping_handle = Builder::new()
            .name("ping_scheduler".to_owned())
            .spawn(move || self1.schedule_pings()).unwrap();

        let server_handle = Builder::new()
            .name("server".to_owned())
            .spawn(move || self2.run_server()).unwrap();

        Ok(vec![ping_handle, server_handle])
    }

    pub fn join(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Join { from: self.config.address };
        self.send_message(msg, address)
    }

    fn schedule_pings(&self) {
        loop {
            {
                let nodes = self.nodes.read().unwrap();
                let nodes_length = nodes.len();
                if nodes_length > 0 {
                    let mut rng = thread_rng();
                    let i = if nodes_length == 1 {
                        0
                    } else {
                        rng.gen_range(0, nodes_length * 10)
                    };
                    let node = nodes.values().nth(i % nodes_length).unwrap();
                    match node.state {
                        State::Dead => continue,
                        _ => self.send_peers(node.address).unwrap(),
                    }
                    println!("pinging {:?}", node);
                }
            }
            sleep(self.config.detection_period);
        }
    }

    fn run_server(&self) {
        let socket = UdpSocket::bind(self.config.address).unwrap();
        socket.set_write_timeout(Some(self.config.timeout)).unwrap();
        let mut buf = [0; 1000];

        loop {
            self.increment_counts().unwrap();
            self.filter_nodes().unwrap();
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
            let mut deserializer = Deserializer::new(&buf[0..number_of_bytes]);
            let msg: Message = Deserialize::deserialize(&mut deserializer).unwrap();

            match msg {
                Ping { from } => self.send_ack(from),
                Join { from } => self.add_node(from),
                Ack  { from } => self.reset_count(from),
                SendPeers { from } => self.update_peers(from),
                _ => continue,
            };
        };
    }

    fn reset_count(&self, from: SocketAddr) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        if let Some(x) = nodes.get_mut(&from.to_string()) {
            x.last_message = 0;
            x.state = State::Alive;
        };
        Ok(())
    }

    pub fn add_node(&self, address: SocketAddr) -> Result<(), ()> {
        let node = Node {
            address: address,
            state: State::Alive,
            last_message: 0,
        };
        let mut ns = self.nodes.write().unwrap();
        let address_str = address.to_string();
        let n = ns.entry(address_str).or_insert(node);
        n.state = State::Alive;
        Ok(())
    }

    fn send_peers(&self, address: SocketAddr) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        let all_nodes = format!("{:?}", nodes.keys());
        let msg = SendPeers(all_nodes);
        self.send_message(msg, address)
    }

    fn filter_nodes(&self) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        for (_k, v) in nodes.iter_mut() {
            if v.last_message > 5 {
                v.state = State::Questionable;
            } else if v.last_message > 25 {
                v.state = State::Dead;
            };
        };
        Ok(())
    }

    fn increment_counts(&self) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        for val in nodes.values_mut() {
            val.last_message += 1;
        }
        Ok(())
    }

    fn send_ping(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Ping { from: self.config.address };
        self.send_message(msg, address)
    }

    fn send_ack(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Ack { from: self.config.address };
        self.send_message(msg, address)
    }

    fn send_message(&self, msg: Message, address: SocketAddr) -> Result<(), ()> {
        let mut buf = Vec::new();
        msg.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let addr = format!("{}:0", self.config.address.ip());
        let socket = UdpSocket::bind(&addr).unwrap();
        socket.set_write_timeout(Some(self.config.timeout)).unwrap();
        socket.send_to(&buf, address).map(|_| ()).unwrap();
        Ok(())
    }
}
