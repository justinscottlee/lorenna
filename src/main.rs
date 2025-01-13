mod hash;

use hash::{Hash, hashv, hash};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

pub struct Block {
    pub previous_hash: Hash, // Hash of the previous block.
    pub hash: Hash, // Hash of this block.
    pub merkle_root: Hash, // Computed merkle root of this block's transactions.

    pub index: u64, // index of the block in the blockchain

    pub leader_signature: Vec<u8>, // Leader's signature of this block.
}

fn main() {
    let mut current_hash = hash("genesis_string".as_bytes());

    // get current instant
    let now = std::time::Instant::now();
    for _ in 1..2000000 {
        current_hash = hash(&current_hash.0);
    }
    // print elapsed time
    println!("{:?}", now.elapsed());
}