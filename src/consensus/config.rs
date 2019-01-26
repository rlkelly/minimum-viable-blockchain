use std::net::SocketAddr;
use std::time::Duration;


pub struct Config {
    pub address: SocketAddr,
    pub timeout: Duration,
    pub detection_period: Duration,
    pub detection_group_size: u16,
}
