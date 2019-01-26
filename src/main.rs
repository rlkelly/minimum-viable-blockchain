#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate crypto;
extern crate futures;
extern crate rand;
extern crate rmp_serde;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

pub mod chain;
pub mod consensus;
pub mod hash;

use self::chain::address::new_keypair;
use self::chain::blockchain::BlockChain;
use self::hash::{hash_vec, Hash};
use std::env;

fn main() {
    let (_my_secret, my_public) = new_keypair();
    let (_receiver_secret, receiver_public) = new_keypair();
    let trans = chain::transaction::Transaction {
        sender: my_public,
        receiver: receiver_public,
        amount: 1.1,
        block: 0,
    };
    let transactions = vec![trans];
    let my_vec: Hash = hash_vec(&transactions);
    println!("{:?}", my_vec);

    BlockChain::new(my_public);
    let (sender_secret, sender_public) = new_keypair();
    let (_receiver_secret, receiver_public) = new_keypair();

    let trans = chain::transaction::Transaction {
        sender: sender_public,
        receiver: receiver_public,
        amount: 1.1,
        block: 0,
    };
    let signed_trans = chain::transaction::SignedTransaction::new(trans, &sender_secret);
    println!("{:?}", signed_trans.verify());

    let args: Vec<String> = env::args().collect();
    consensus::gossip::GossipClient::run(args[1].parse().unwrap());
}
