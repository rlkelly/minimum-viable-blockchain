use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde_json::to_string;

pub const HASH_SIZE: usize = 32;
pub type Hash = [u8; HASH_SIZE];
pub const EMPTY_HASH: Hash = [0u8; HASH_SIZE];
pub const DIFFICULTY: [u8; 2] = [0, 0];

pub fn hash_string(hash_str: &str) -> Hash {
    let mut sha = Sha256::new();
    let mut array = [0; HASH_SIZE];
    sha.input_str(hash_str);
    sha.result(&mut array);
    array
}

pub fn hash_struct<T>(transactions: &T) -> Hash
where
    T: serde::Serialize,
{
    let mut sha = Sha256::new();
    let mut array = [0; HASH_SIZE];
    let block_string = to_string(&transactions).unwrap();
    sha.input_str(&block_string);
    sha.result(&mut array);
    array
}

pub fn hash_vec<T>(transactions: &Vec<T>) -> Hash
where
    T: serde::Serialize,
{
    let mut sha = Sha256::new();
    let mut array = [0; HASH_SIZE];
    let block_string = to_string(&transactions).unwrap();
    sha.input_str(&block_string);
    sha.result(&mut array);
    array
}
