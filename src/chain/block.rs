use chrono::prelude::*;
use rand::prelude::*;

use super::address::Public;
use super::header::Header;
use super::transaction::SignedTransaction;
use crate::hash::{DIFFICULTY, Hash, hash_vec};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn new(prev_block: &Block, address: Public) -> Block {
        let transactions: Vec<SignedTransaction> = Vec::new();
        Block {
            header: Header::new(&prev_block.header, address),
            transactions,
        }
    }

    pub fn add_transaction(&mut self, transaction: SignedTransaction) {
        if transaction.verify() {
            if !self.transactions.contains(&transaction) {
                self.transactions.push(transaction);
                self.update_header();
            }
        }
    }

    pub fn update_header(&mut self) {
        self.header.transactions_hash = hash_vec(&self.transactions);
        let utc: DateTime<Utc> = Utc::now();
        self.header.timestamp = utc.timestamp();
    }

    pub fn prove_work(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let nonce: u64 = rng.gen();
        let my_vec: Hash = hash_vec(&self.transactions);
        if my_vec[..4] == DIFFICULTY {
            self.header.nonce = nonce;
            true
        } else {
            false
        }
    }

    pub fn filter_transactions(&mut self) {
        &self.transactions.retain(|trans| trans.verify());
    }

    pub fn verify(&self) -> bool {
        for transaction in &self.transactions {
            if transaction.verify() == false {
                return false
            }
        }
        return true
    }

    pub fn genesis() -> Block {
        let transactions: Vec<SignedTransaction> = vec!();
        let header = Header::genesis();
        Block{
            header,
            transactions,
        }
    }
}
