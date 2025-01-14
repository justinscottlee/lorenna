use sha2::{Digest, Sha256};
use std::fmt;
use bs58;

const HASH_SIZE: usize = 32;
#[derive(Clone, Copy)]
pub struct Hash(pub [u8; HASH_SIZE]);

/*
// Print Hash as a base58 string.
impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}
*/

// Print Hash as a hex string.
impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
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