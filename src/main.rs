mod hash;
mod merkle;

use hash::{Hash, hashv};
use merkle::MerkleTree;

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
    let mut items: [&[u8]; 9] = [&[0], &[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8]];
    let mut mt = MerkleTree::new(&items);
}