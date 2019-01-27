use crypto::ed25519::keypair;
use rand::Rng;

use crate::hash::Hash;
use bitcoin::util::base58;

pub type Public = Hash;
pub type Private = [u8; 64];

pub fn gen_seed() -> Public {
    let mut rng = rand::thread_rng();
    let seed: Hash = rng.gen();
    seed
}

pub fn generate_keypair(seed: &Public) -> (Private, Public) {
    keypair(seed)
}

pub fn generate_keypair_strings(seed: &Public) -> (String, String) {
    let (public, private) = keypair(seed);
    (base58::encode_slice(&public), base58::encode_slice(&private))
}

pub fn new_keypair() -> (Private, Public) {
    generate_keypair(&gen_seed())
}
