mod hash;
mod merkle;

use hash::{Hash, hashv};
use merkle::MerkleTree;

fn main() {
    for i in 0..=256 {
        println!("Testing {} element tree", i);
        let mut data = Vec::new();
        for j in 0..i {
            data.push(vec!(j as u8));
        }
        let tree = MerkleTree::new(&data);
        for j in 0..i {
            let proof = tree.generate_proof(j);
            for k in 0..i {
                let res = proof.verify_proof(&data[k], tree.root().unwrap());
                if k == j {
                    assert!(res);
                } else {
                    assert!(!res);
                }
            }
        }
    }
}