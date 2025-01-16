use crate::{Hash, hashv};
use fast_math;

/*
// Create distinction between leaf and internal nodes to protect against second-preimage attacks.
// https://flawed.net.nz/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack/
*/
macro_rules! hash_leaf {
    ($leaf:expr) => {
        hashv(&[&[0], $leaf])
    };
}

macro_rules! hash_internal {
    ($left:expr, $right:expr) => {
        hashv(&[&[1], &$left.0, &$right.0])
    };
}

pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub leaf_count: usize,
}

#[derive(Clone)]
pub struct MerkleProof {
    pub proof: Vec<Hash>,
    pub leaf_index: usize,
    pub leaf_count: usize,
}

impl MerkleProof {
    pub fn verify_proof(&self, leaf: &[u8], root: &Hash) -> bool {
        let mut hash = hash_leaf!(leaf);

        let mut index = self.leaf_index;
        let mut level_size = self.leaf_count;
        for sibling in &self.proof {
            while index % 2 == 0 && index == level_size - 1 {
                hash = hash_internal!(hash, hash);
                index /= 2;
                level_size = (level_size + 1) / 2;
            }

            if index % 2 == 0 {
                hash = hash_internal!(hash, sibling);
            } else {
                hash = hash_internal!(sibling, hash);
            }

            level_size = (level_size + 1) / 2;
            index /= 2;
        }

        hash == *root
    }
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]>>(items: &[T]) -> Self {
        let mut tree = MerkleTree {
            nodes: Vec::with_capacity(fast_math::log2_raw(items.len() as f32) as usize + 2 * items.len() + 1),
            leaf_count: items.len(),
        };

        for item in items {
            tree.nodes.push(hash_leaf!(item.as_ref()));
        }

        let mut level_size = tree.leaf_count;
        let mut level_start_index = 0;
        while level_size > 1 {
            let mut next_level = Vec::with_capacity((level_size + 1) / 2);
            for i in (0..level_size).step_by(2) {
                let left = tree.nodes[i + level_start_index];
                let right = if i == level_size - 1 {
                    tree.nodes[i + level_start_index]
                } else {
                    tree.nodes[i + level_start_index + 1]
                };
                next_level.push(hash_internal!(left, right));
            }
            level_start_index += level_size;
            level_size = next_level.len();
            tree.nodes.append(&mut next_level);
        }

        tree
    }
    
    pub fn generate_proof(&self, leaf_index: usize) -> MerkleProof {
        let mut proof = Vec::with_capacity(fast_math::log2_raw(self.leaf_count as f32) as usize + 1);
        let mut index = leaf_index;

        let mut level_start_index = 0;
        let mut level_size = self.leaf_count;
        while level_size > 1 {
            let sibling_index = if index % 2 == 0 {
                index + 1
            } else {
                index - 1
            };

            if sibling_index < level_size {
                let sibling = self.nodes[level_start_index + sibling_index];
                proof.push(sibling);
            }

            index /= 2;
            level_start_index += level_size;
            level_size = (level_size + 1) / 2;
        }

        MerkleProof {
            proof,
            leaf_index,
            leaf_count: self.leaf_count,
        }
    }

    pub fn root(&self) -> Option<&Hash> {
        self.nodes.last()
    }
}