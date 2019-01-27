use bitcoin::util::base58;
use rmp_serde::Serializer;
use serde::Serialize;
use std::io;
use std::io::Write;
use std::net;
use std::net::SocketAddr;

use crate::consensus::message::Message;
use crate::consensus::peers::serialize_message;
use crate::chain::transaction::{Transaction, SignedTransaction};

const HOST: &str = "127.0.0.1:3000";
const MINE: &str = "127.0.0.1:4001";

fn slice_to_arr32<T>(slice: &[T]) -> Option<&[T; 32]> {
    if slice.len() == 32 {
        Some(unsafe { &*(slice as *const [T] as *const [T; 32]) })
    } else {
        None
    }
}

pub fn run_wallet() {
    loop {

        print!("public key: ");
        let mut public_key_buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut public_key_buffer).expect("reading from stdin failed");

        print!("private key: ");
        let mut priv_key_buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut priv_key_buffer).expect("reading from stdin failed");

        print!("receiver public key: ");
        let mut receiver_buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut receiver_buffer).expect("reading from stdin failed");

        print!("amount: ");
        let mut amount_buffer = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut amount_buffer).expect("reading from stdin failed");

        print!("block_number: ");
        let mut block_number = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut block_number).expect("reading from stdin failed");

        let sender_public_key = base58::from(&public_key_buffer.trim()).expect("invalid String");
        let receiver_public_key = base58::from(&receiver_buffer.trim()).expect("invalid String");
        let sender_private_key = base58::from(&priv_key_buffer.trim()).expect("invalid String");

        let transaction = Transaction {
            sender: *slice_to_arr32(&sender_public_key).unwrap(),
            receiver: *slice_to_arr32(&receiver_public_key).unwrap(),
            amount: amount_buffer.trim().parse().expect("invalid amount"),
            block: block_number.trim().parse().expect("invalid block"),
        };

        let signed_transaction = SignedTransaction::new(transaction, &sender_private_key);
        if signed_transaction.verify_signature() {
            let socket = net::UdpSocket::bind(MINE).expect("failed to bind host socket");
            let address: SocketAddr = MINE.parse().expect("invalid socket");

            let mut buf = Vec::new();
            signed_transaction.serialize(&mut Serializer::new(&mut buf)).expect("Serialization Error");

            let msg : Message = Message::Transaction {
                transaction: signed_transaction,
                from: address,
            };
            socket.send_to(&serialize_message(msg), &HOST).unwrap();
            println!("Transaction Sent!\n");
        } else {
            println!("Invalid Transaction");
        }
    }
}
