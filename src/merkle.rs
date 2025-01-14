use crate::{Hash, hashv};

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

impl MerkleTree {
    pub fn new(items: &[&[u8]]) -> Self {
        let mut tree = MerkleTree {
            nodes: Vec::with_capacity(2 * items.len() - 1),
            leaf_count: items.len(),
        };

        for item in items {
            let leaf = hash_leaf!(item);
            tree.nodes.push(leaf);
        }

        let mut level_size = tree.leaf_count;
        let mut level_start_index = 0;
        while level_size > 1 {
            let mut next_level = Vec::with_capacity(level_size);
            for i in (0..level_size).step_by(2) {
                let left = tree.nodes[i + level_start_index];
                let right = if i + 1 < level_size {
                    tree.nodes[i + 1 + level_start_index]
                } else {
                    tree.nodes[i + level_start_index]
                };
                let internal = hash_internal!(left, right);
                next_level.push(internal);
            }
            level_start_index += level_size;
            level_size = next_level.len();
            tree.nodes.append(&mut next_level);
        }

        tree
    }
    
    pub fn generate_proof(&self, leaf_index: usize) -> Vec<Hash> {
        let mut proof = Vec::new();
        let mut index = leaf_index;

        let mut level_start_index = 0;
        let mut level_size = self.leaf_count;
        while level_size > 1 {
            let sibling_index = if index % 2 == 0 {
                index + 1
            } else {
                index -1
            };

            if sibling_index < level_size {
                let sibling = self.nodes[level_start_index + sibling_index];
                proof.push(sibling);
            }

            index /= 2;
            level_start_index += level_size;
            level_size = (level_size + 1) / 2;
        }

        proof
    }

    pub fn get_root(&self) -> Option<&Hash> {
        self.nodes.last()
    }
}