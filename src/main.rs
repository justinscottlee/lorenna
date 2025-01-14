mod hash;
mod merkle;

use hash::{Hash, hashv};
use merkle::MerkleTree;

fn main() {
    let items: [&[u8]; 9] = [&[0], &[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8]];
    let mt = MerkleTree::new(&items);
    let proof = mt.generate_proof(3);
    println!("Proof:");
    for hash in proof {
        println!("{}", hash);
    }
}