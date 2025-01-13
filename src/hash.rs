use sha2::{Digest, Sha256};
use std::fmt;
use bs58;

const HASH_SIZE: usize = 32;
pub struct Hash(pub [u8; HASH_SIZE]);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

pub fn hashv(vals: &[&[u8]]) -> Hash {
    let mut hasher = Sha256::new();
    for val in vals {
        hasher.update(val);
    }
    Hash(hasher.finalize_reset().into())
}

pub fn hash(val: &[u8]) -> Hash {
    hashv(&[val])
}