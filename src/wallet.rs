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
const MINE: &str = "127.0.0.1:3001";

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

        let sender_public_key: [u8; 32] = *slice_to_arr32(public_key_buffer.as_bytes()).expect("invalid length");
        let receiver_public_key: [u8; 32] = *slice_to_arr32(receiver_buffer.as_bytes()).expect("invalid length");

        let transaction = Transaction {
            sender: sender_public_key,
            receiver: receiver_public_key,
            amount: amount_buffer.parse().expect("invalid amount"),
            block: block_number.parse().expect("invalid block"),
        };

        let signed_transaction = SignedTransaction::new(transaction, &priv_key_buffer.as_bytes());
        if signed_transaction.verify() {
            let socket = net::UdpSocket::bind(MINE).expect("failed to bind host socket");
            let address: SocketAddr = HOST.parse().unwrap();

            let mut buf = Vec::new();
            signed_transaction.serialize(&mut Serializer::new(&mut buf)).unwrap();

            let msg : Message = Message::Transaction {
                data: std::str::from_utf8(&buf).unwrap().to_string(),
                from: address,
            };

            socket.send_to(&serialize_message(msg), &HOST).unwrap();
        } else {
            println!("Invalid Transaction");
        }

    }
}
