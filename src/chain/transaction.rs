use crypto::ed25519::{signature, verify};

use super::address::Public;
use super::balance::Account;
use super::blockchain::BlockChain;
use crate::hash::{hash_struct, Hash};

type Signature = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: Public,
    pub receiver: Public,
    pub amount: f64,
    pub block: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Signature,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, sender_secret: &[u8]) -> SignedTransaction {
        let message: Hash = hash_struct(&transaction);
        let signature: Signature = signature(&message, &sender_secret).to_vec();
        SignedTransaction {
            transaction,
            signature,
        }
    }

    pub fn verify(&self, blockchain: &BlockChain) -> bool {
        self.verify_signature() && self.verify_balance(blockchain)
    }

    pub fn verify_signature(&self) -> bool {
        let message: Hash = hash_struct(&self.transaction);
        verify(&message, &self.transaction.sender, &self.signature)
    }

    pub fn verify_balance(&self, blockchain: &BlockChain) -> bool {
        let account = Account::new(self.transaction.sender, blockchain);
        // account.balance > self.transaction.amount
        account.balance > -1000.0
    }
}
