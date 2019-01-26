extern crate rand;
extern crate serde;

use crate::consensus::config::Config;

use rand::{thread_rng, Rng};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::sync::{Arc};
use std::time::Duration;

use crate::peers::Peers;


pub struct GossipClient;

impl GossipClient {
    pub fn run(addr: u16) {
        let config = Config {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), addr),
            timeout: Duration::from_millis(5000),
            detection_period: Duration::from_millis(3000),
            detection_group_size: 2,
        };
        let g = Arc::new(Peers::with_config(config));
        let handles = Peers::start(g.clone()).unwrap_or_else(|_| panic!("Error"));
        g.add_node(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000));
        g.add_node(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3001));
        g.join(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), addr)).unwrap();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
