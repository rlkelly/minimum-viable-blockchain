use crypto::ed25519::{keypair};
use rand::Rng;

use crate::hash::Hash;


pub type Public = Hash;
pub type Private = [u8; 64];

pub fn gen_seed() -> Public {
    let bytes = rand::thread_rng().gen::<Public>();
    bytes
}

pub fn generate_keypair(seed: &Public) -> (Private, Public) {
    keypair(seed)
}

pub fn new_keypair() -> ([u8; 64], [u8; 32]) {
    generate_keypair(&gen_seed())
}
