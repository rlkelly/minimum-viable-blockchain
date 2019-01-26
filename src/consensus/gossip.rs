extern crate rand;
extern crate serde;

use crate::consensus::config::Config;

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::sync::{Arc};
use std::time::Duration;

use crate::consensus::peers::Peers;


pub struct GossipClient;

impl GossipClient {
    pub fn run(addr: u16) {
        let config = Config {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), addr),
            timeout: Duration::from_millis(1000),
            detection_period: Duration::from_millis(1000),
            detection_group_size: 2,
        };
        let g = Arc::new(Peers::with_config(config));
        let handles = Peers::start(g.clone()).unwrap_or_else(|_| panic!("Error"));

        // ensure you join the network
        g.join(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), addr)).unwrap();
        // the default node
        g.add_node(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000)).unwrap();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
