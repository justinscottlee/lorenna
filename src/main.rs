use sha2::{Digest, Sha256};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

pub struct Block {
    pub timestamp: [u8; 32], // Timestamp hash.
    pub transactions: Vec<Transaction>, // A vector of this block's transactions.

    pub previous_hash: [u8; 32], // Hash of the previous block.
    pub hash: [u8; 32], // Hash of this block.

    pub merkle_root: [u8; 32], // Computed merkle root of this block's transactions.

    pub index: u64, // index of the block in the blockchain

    pub leader_signature: Vec<u8>, // Leader's signature of this block.
}

fn main() {
    // Get current time in milliseconds.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut current_hash = Sha256::digest(b"Genesis");
    let mut hasher = Sha256::new();
    for _ in 1..5000000 {
        hasher.update(&current_hash);
       current_hash = hasher.finalize_reset();
    }

    // Get end time.
    let end = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("Time taken: {}", end - timestamp);
}
