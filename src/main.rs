#[macro_use]
extern crate serde_derive;

extern crate bitcoin;
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
pub mod wallet;

use self::chain::address::new_keypair;
use self::chain::blockchain::BlockChain;
use self::hash::{hash_vec, Hash};
use std::env;
use bitcoin::util::base58;

fn main() {
    let (my_secret, my_public) = new_keypair();
    println!("public:{}", base58::encode_slice(&my_public));
    println!("private:{}", base58::encode_slice(&my_secret));
    println!("back:{:?}", base58::from(&base58::encode_slice(&my_secret)));
    println!("{:?}:{:?}", my_secret.to_vec(), my_public.to_vec());
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
    if args.len() >= 2 {
        consensus::gossip::GossipClient::run(args[1].parse().expect("Invalid Port"));
    } else {
        consensus::gossip::GossipClient::run(3000);
    }
}
