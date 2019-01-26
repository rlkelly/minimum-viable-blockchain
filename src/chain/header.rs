use chrono::prelude::*;

use crate::hash::{Hash, EMPTY_HASH, DIFFICULTY};
use super::address::Public;


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub index: u64,
    pub previous_hash: Hash,
    pub timestamp: i64,
    pub transactions_hash: Hash,
    pub nonce: u64,
    pub hash: Hash,
    pub difficulty: [u8; 4],
    pub coinbase: Public,
}

impl Header {
    pub fn new(prev_header: &Header, coinbase: Public) -> Header {
        let utc: DateTime<Utc> = Utc::now();
        let index: u64 = prev_header.index + 1;
        let previous_hash: Hash = prev_header.hash;
        let timestamp: i64 = utc.timestamp();
        let nonce: u64 = 0;
        Header {
            index,
            previous_hash,
            timestamp,
            transactions_hash: EMPTY_HASH,
            nonce,
            hash: EMPTY_HASH,
            difficulty: DIFFICULTY,
            coinbase,
        }
    }

    pub fn genesis() -> Header {
        Header {
            index: 0,
            previous_hash: EMPTY_HASH,
            timestamp: 0,
            transactions_hash: EMPTY_HASH,
            nonce: 0,
            hash: EMPTY_HASH,
            difficulty: DIFFICULTY,
            coinbase: EMPTY_HASH,
        }
    }
}
