mod hash;
mod merkle;
mod hashclock;

use hash::{Hash, hashv, hash};
use hashclock::HashClock;
use merkle::MerkleTree;

fn main() {
    // start time
    let start = std::time::Instant::now();
    let transactions = hash(&[214]);
    let mut clock = HashClock::new(hash(&[0]));
    println!("Initial hash: {}", clock.checkpoints[0]);
    for _ in 0..70 {
        clock.tick(transactions);
        println!("Hash after tick: {}", clock.checkpoints[clock.tick as usize]);
    }
    println!("Final hash: {}", clock.checkpoints[64]);
    // end time
    let end = std::time::Instant::now();
    println!("Time elapsed: {:?}", end.duration_since(start));

    /*
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
    }*/
}