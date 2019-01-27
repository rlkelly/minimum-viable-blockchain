use std::net::SocketAddr;
use std::time::Duration;
use crate::chain::address::Public;

pub struct Config {
    pub address: SocketAddr,
    pub wallet: Public,
    pub timeout: Duration,
    pub detection_period: Duration,
    pub detection_group_size: u16,
}
