use std::net::SocketAddr;

use crate::chain::block::Block;
use crate::chain::transaction::SignedTransaction;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Message {
    Ping { from: SocketAddr },
    SendPeers { peers: String, from: SocketAddr },
    PingReq { from: SocketAddr, to: SocketAddr },
    Ack { from: SocketAddr },
    Join { from: SocketAddr },
    Transaction { transaction: SignedTransaction, from: SocketAddr },
    NewBlock { block: Block, from: SocketAddr },
}
