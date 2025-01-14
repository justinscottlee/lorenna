use crate::{Hash, hashv};

pub struct MerkleTree {
    nodes: Vec<Hash>,
    leaf_count: usize,
}

impl MerkleTree {
    pub fn new(items: &[&[u8]]) -> Self {
        let mut tree = MerkleTree {
            nodes: Vec::with_capacity(2 * items.len() - 1),
            leaf_count: items.len(),
        };

        for item in items {
            let leaf = hashv(&[&[0], item]);
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
                next_level.push(hashv(&[&[1], &left.0, &right.0]));
            }
            level_start_index += level_size;
            level_size = next_level.len();
            tree.nodes.append(&mut next_level);
        }

        tree
    }
}