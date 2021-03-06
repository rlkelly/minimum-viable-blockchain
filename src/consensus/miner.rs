extern crate rand;
extern crate serde;

use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::net::AddrParseError;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{Builder, JoinHandle};

use crate::consensus::config::Config;
use crate::chain::blockchain::BlockChain;
use crate::consensus::message::Message::{self, Ack, Join, NewBlock, Ping, SendPeers, Transaction};
use crate::consensus::node::{Node, State};

pub struct Miner {
    config: Config,
    nodes: RwLock<BTreeMap<String, Node>>,
    blockchain: RwLock<BlockChain>,
}

impl Miner {
    pub fn with_config(config: Config) -> Miner {
        let wallet = config.wallet.clone();
        Miner {
            config: config,
            nodes: RwLock::new(BTreeMap::new()),
            blockchain: RwLock::new(BlockChain::new(wallet)),
        }
    }

    pub fn start(miner: Arc<Self>) -> Result<Vec<JoinHandle<()>>, ()> {
        let self1 = miner.clone();
        let self2 = self1.clone();

        let ping_handle = Builder::new()
            .name("ping_scheduler".to_owned())
            .spawn(move || self1.schedule_pings())
            .unwrap();

        let server_handle = Builder::new()
            .name("server".to_owned())
            .spawn(move || self2.run_server())
            .unwrap();

        Ok(vec![ping_handle, server_handle])
    }

    pub fn join(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Join {
            from: self.config.address,
        };
        self.send_message(msg, address)
    }

    fn schedule_pings(&self) {
        loop {
            {
                self.filter_nodes().unwrap();
                let nodes = self.nodes.read().unwrap();
                let nodes_length = nodes.len();
                if nodes_length > 0 {
                    let node = nodes.values().max_by_key(|n| n.last_attempt.fetch_add(0, Ordering::Relaxed)).unwrap();
                    node.last_attempt.swap(0, Ordering::Relaxed);
                    node.last_response.fetch_add(1, Ordering::Relaxed);
                    match node.state {
                        State::Alive => self.send_peers(node.address).unwrap(),
                        State::Questionable => self.send_ping(node.address).unwrap(),
                        State::Dead => self.send_ping(node.address).unwrap(),
                    }
                    println!("pinging {:?}", node);
                }
            }
            let mut blockchain = self.blockchain.write().unwrap();
            for _ in 1..5000 {
                let valid = blockchain.prove_work();
                if valid {
                    let msg = NewBlock { block: blockchain.current_block.clone(), from: self.config.address };
                    self.send_all(msg);
                    blockchain.add_current_block(self.config.wallet);
                }
            }
        }
    }

    fn run_server(&self) {
        let socket = UdpSocket::bind(self.config.address).unwrap();
        socket.set_write_timeout(Some(self.config.timeout)).unwrap();
        let mut buf = [0; 1000];

        loop {
            let (number_of_bytes, _src_addr) =
                socket.recv_from(&mut buf).expect("Didn't receive data");
            let mut deserializer = Deserializer::new(&buf[0..number_of_bytes]);
            let msg: Message = Deserialize::deserialize(&mut deserializer).unwrap();

            match msg {
                Ping { from } => self.send_ack(from),
                Join { from } => self.add_node(from),
                Ack { from } => self.reset_count(from),
                SendPeers { peers, from } => {
                    self.send_ack(from).unwrap();
                    self.update_peers(peers)
                }
                Transaction { transaction, from } => {
                    println!("received transaction");
                    let mut blockchain = self.blockchain.write().unwrap();
                    // if it's a new transaction, send to everyone
                    if blockchain.add_transaction(transaction.clone()) {
                        self.send_all(Transaction{transaction, from})
                    }
                    Ok(())
                },
                NewBlock { block, from } => {
                    println!("received new block");
                    let mut blockchain = self.blockchain.write().unwrap();
                    if block.header.index == blockchain.current_block.header.index {
                        if blockchain.receive_new_block(block.clone(), self.config.wallet) {
                            self.send_all(NewBlock{block, from})
                        }
                    }
                    println!("current index: {}", blockchain.current_block.header.index);
                    Ok(())

                },
                _ => continue,
            }.unwrap();
        }
    }

    fn send_all(&self, msg: Message) {
        let nodes = self.nodes.read().unwrap();
        for node in nodes.values().filter(|x| x.state != State::Dead) {
            if node.address != self.config.address {
                self.send_message(msg.clone(), node.address).unwrap();
            }
        }
    }

    fn reset_count(&self, from: SocketAddr) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        if let Some(x) = nodes.get_mut(&from.to_string()) {
            x.last_response = Arc::new(AtomicUsize::new(0));
            x.state = State::Alive;
        };
        Ok(())
    }

    pub fn add_node(&self, address: SocketAddr) -> Result<(), ()> {
        let node = Node {
            address: address,
            state: State::Alive,
            last_attempt: Arc::new(AtomicUsize::new(0)),
            last_response: Arc::new(AtomicUsize::new(0)),
        };
        let mut ns = self.nodes.write().unwrap();
        let address_str = address.to_string();
        let n = ns.entry(address_str).or_insert(node);
        n.state = State::Alive;
        Ok(())
    }

    fn send_peers(&self, address: SocketAddr) -> Result<(), ()> {
        let nodes = self.nodes.read().unwrap();
        let mut all_nodes = String::from("");
        for (key, _) in nodes.iter() {
            all_nodes.push_str(key);
            all_nodes.push_str(";");
        }

        let msg = SendPeers {
            peers: all_nodes,
            from: self.config.address,
        };
        self.send_message(msg, address)
    }

    fn filter_nodes(&self) -> Result<(), ()> {
        let mut nodes = self.nodes.write().unwrap();
        for (_k, v) in nodes.iter_mut() {
            let val = v.last_attempt.fetch_add(1, Ordering::Relaxed);
            if val > 5 {
                v.state = State::Questionable;
            } else if val > 10 {
                v.state = State::Dead;
            };
        }
        Ok(())
    }

    fn send_ping(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Ping {
            from: self.config.address,
        };
        self.send_message(msg, address)
    }

    fn send_ack(&self, address: SocketAddr) -> Result<(), ()> {
        let msg = Ack {
            from: self.config.address,
        };
        self.send_message(msg, address)
    }

    fn update_peers(&self, peers: String) -> Result<(), ()> {
        for peer in peers.split(";") {
            let peer_address: Result<SocketAddr, AddrParseError> = peer.parse();
            match peer_address {
                Ok(address) => self.add_node(address).unwrap(),
                Err(_) => (),
            }
        }
        Ok(())
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

pub fn serialize_message(msg: Message) -> Vec<u8> {
    let mut buf = Vec::new();
    msg.serialize(&mut Serializer::new(&mut buf)).unwrap();
    buf
}
