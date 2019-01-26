#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate crypto;
extern crate futures;
extern crate rand;
extern crate rmp_serde;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

use std::env;

pub mod chain;
pub mod hash;
pub mod consensus;


fn main() {
    let (my_secret, my_public) = chain::address::new_keypair();
    let (receiver_secret, receiver_public) = chain::address::new_keypair();
    let trans = chain::transaction::Transaction{
        sender: my_public,
        receiver: receiver_public,
        amount: 1.1,
        block: 0,
    };
    let transactions = vec![trans,];
    let my_vec: hash::Hash = hash::hash_vec(&transactions);
    println!("{:?}", my_vec);

    chain::blockchain::BlockChain::new(my_public);
    let (sender_secret, sender_public) = chain::address::new_keypair();
    let (receiver_secret, receiver_public) = chain::address::new_keypair();

    let trans = chain::transaction::Transaction{
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
