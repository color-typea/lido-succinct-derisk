//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolType};

use log::{debug, warn};

use lido_derisk_lib::hashing::merkleization::MerkleTreeBuilder;
use lido_derisk_lib::hashing::sha256::{hash, ZEROHASHES};
use lido_derisk_lib::hashing::types::HashElement;
use lido_derisk_lib::ssz::validator::Validator;

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(bytes32,)
};

// This is a constant in Ethereum network
const VALIDATORS_MAX_SIZE_LOG2: usize = 40;
// Validator merkleization is an actual sha2-256 hash, so the full tree height is used
const VALIDATORS_TARGET_TREE_HEIGHT: usize = VALIDATORS_MAX_SIZE_LOG2;

const BALANCES_PER_LEAF_LOG2: usize = 2;
const BALANCES_PER_LEAF: usize = 1 << BALANCES_PER_LEAF_LOG2; // // 2 ** BALANCES_PER_LEAF_LOG2
                                                              // Balances are "packed" together (4 balances in one merkle leaf), so their target tree height is smaller by 2
const BALANCES_TARGET_TREE_HEIGHT: usize = VALIDATORS_MAX_SIZE_LOG2 - BALANCES_PER_LEAF_LOG2;

#[sp1_derive::cycle_tracker]
fn validators_merkle(builder: &MerkleTreeBuilder, validators: &Vec<Validator>) -> HashElement {
    println!("cycle-tracker-start: individual validator merkle");
    let merkle_leaves: Vec<HashElement> = validators
        .into_iter()
        .map(|validator| validator.compute_merkle_root(builder))
        .collect();
    println!("cycle-tracker-end: individual validator merkle");

    debug!("Validators Leafs: {:?}", merkle_leaves);

    println!("cycle-tracker-start: validators merkle root");
    let result = builder.merkleize(
        &merkle_leaves,
        Some(VALIDATORS_TARGET_TREE_HEIGHT),
        true,
        Some(validators.len()),
    );
    println!("cycle-tracker-end: validators merkle root");
    return result;
}

fn main() {
    println!("cycle-tracker-start: read-args");
    let input = sp1_zkvm::io::read::<Vec<Validator>>();
    let expected_merkle_root = sp1_zkvm::io::read::<HashElement>();
    debug!("Input : {:?}", input);
    debug!("Hash  : {:?}", expected_merkle_root);
    println!("cycle-tracker-end: read-args");

    let merkle_tree_builder = MerkleTreeBuilder::new(&hash, &ZEROHASHES);

    println!("cycle-tracker-start: merkleizing");
    let merkle_root = validators_merkle(&merkle_tree_builder, &input);
    println!("cycle-tracker-end: merkleizing");

    println!("cycle-tracker-start: debug-logs");
    debug!("Expected :{:?}", expected_merkle_root);
    debug!("Actual   :{:?}", merkle_root);
    println!("cycle-tracker-end: debug-logs");

    println!("cycle-tracker-start: assertions");
    assert_eq!(&expected_merkle_root, &merkle_root);
    println!("cycle-tracker-end: assertions");

    println!("cycle-tracker-start: commitments");

    // Encocde the public values of the program.
    // let public_values: PublicValuesTuple = merkle_root;
    let bytes = PublicValuesTuple::abi_encode(&(merkle_root,));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);

    // sp1_zkvm::io::commit(&merkle_root);
    println!("cycle-tracker-end: commitments");
}
