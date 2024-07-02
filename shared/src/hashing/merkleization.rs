use crate::hashing::types::{HashElement, HashFunction, Zerohashes, MAX_TREE_HEIGHT};
use crate::hashing::utils::lift_u64;

use log::debug;
pub struct MerkleTreeBuilder<'a> {
    hash_fn: &'a HashFunction,
    zerohashes: &'a Zerohashes,
}

type MerkleRoot = (HashElement, usize);

impl<'a> MerkleTreeBuilder<'a> {
    pub fn new(hash: &'a HashFunction, zerohashes: &'a Zerohashes) -> Self {
        return Self {
            hash_fn: hash,
            zerohashes: zerohashes,
        };
    }

    // #[sp1_derive::cycle_tracker]
    pub fn hash(&self, a: &HashElement, b: &HashElement) -> HashElement {
        (self.hash_fn)(a, b)
    }

    // #[sp1_derive::cycle_tracker]
    fn hash_layer(&self, input: &Vec<HashElement>, layer: usize) -> MerkleRoot {
        println!("cycle-tracker-start: hash-layer {}", layer);
        let size = input.len();
        // debug!("=== Layer {:?} ===", layer);
        // for leaf in input {
        //     debug!("\t{:x?}", hex::encode(leaf));
        // }
        if size == 1 {
            println!("cycle-tracker-end: hash-layer {}", layer);
            return (input[0], layer);
        }

        let next_layer_size: usize = (size / 2) + (size % 2);
        let mut next_layer: Vec<HashElement> = Vec::with_capacity(next_layer_size);

        for leaf_index in 0..(size / 2) {
            next_layer.push(self.hash(&input[2 * leaf_index], &input[2 * leaf_index + 1]));
        }

        if size % 2 != 0 {
            next_layer.push(self.hash(&input[size - 1], &self.zerohashes[layer]));
        }
        println!("cycle-tracker-end: hash-layer {}", layer);
        self.hash_layer(&next_layer, layer + 1)
    }

    #[sp1_derive::cycle_tracker]
    fn mix_in_size(&self, root: &HashElement, size: u64) -> HashElement {
        self.hash(&root, &lift_u64(size))
    }

    #[sp1_derive::cycle_tracker]
    fn expand_merkle_to_height(&self, merkle: MerkleRoot, target_height: usize) -> MerkleRoot {
        let (mut current_hash, current_height) = merkle;

        assert!(
            current_height <= target_height,
            "Current tree heigth is larger than target height"
        );
        assert!(
            target_height <= MAX_TREE_HEIGHT,
            "Target heigth is above max supported"
        );

        for height in current_height..target_height {
            current_hash = self.hash(&current_hash, &self.zerohashes[height]);
        }
        (current_hash, target_height)
    }

    #[sp1_derive::cycle_tracker]
    pub fn merkleize(
        &self,
        leaves: &Vec<HashElement>,
        target_tree_height: Option<usize>,
        with_size: bool,
        size_override: Option<usize>,
    ) -> HashElement {
        let actual_values_merkle_tree = self.hash_layer(leaves, 0);
        let expanded_merkle = match target_tree_height {
            Some(height) => self.expand_merkle_to_height(actual_values_merkle_tree, height),
            None => actual_values_merkle_tree,
        };
        let (merkle_root, _height) = expanded_merkle;

        if with_size {
            let size: u64 = size_override.unwrap_or(leaves.len()).try_into().unwrap();
            return self.mix_in_size(&merkle_root, size);
        } else {
            return merkle_root;
        }
    }
}
