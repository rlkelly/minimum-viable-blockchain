use std::net::SocketAddr;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Ping { from: SocketAddr },
    SendPeers { peers: String, from: SocketAddr },
    PingReq { from: SocketAddr, to: SocketAddr },
    Ack { from: SocketAddr },
    Join { from: SocketAddr },
    Transaction { data: String, from: SocketAddr },
}
