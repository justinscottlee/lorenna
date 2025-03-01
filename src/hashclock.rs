use crate::{Hash, hashv};

const HASHES_PER_SECOND: u32 = 5_000_000;
const TICKS_PER_SECOND: u32 = 128;
const HASHES_PER_TICK: u32 = HASHES_PER_SECOND / TICKS_PER_SECOND;
const TICKS_PER_BLOCK: u32 = 64;

pub struct HashClock {
    pub checkpoints: Vec<Hash>,
    pub tick: u32,
}

impl HashClock {
    pub fn new(hash: Hash) -> Self {
        let mut checkpoints = Vec::with_capacity((TICKS_PER_BLOCK + 1) as usize);
        checkpoints.push(hash);
        HashClock {
            checkpoints: checkpoints,
            tick: 0,
        }
    }

    pub fn tick(&mut self, transactions: Hash) -> Option<Hash>{
        if self.tick == TICKS_PER_BLOCK {
            return Some(self.checkpoints.last().unwrap().clone())
        }
        let mut hash = self.checkpoints.last().unwrap().clone();
        for _ in 0..(HASHES_PER_TICK - 1) {
            hash = hashv(&[&hash.0]);
        }

        if self.tick == TICKS_PER_BLOCK - 1{
            hash = hashv(&[&hash.0, &transactions.0]);
        } else {
            hash = hashv(&[&hash.0]);
        }

        self.tick += 1;
        self.checkpoints.push(hash);
        if self.tick == TICKS_PER_BLOCK {
            Some(hash)
        } else {
            None
        }
    }

    pub fn verify(&self) -> bool {
        unimplemented!()
    }
}