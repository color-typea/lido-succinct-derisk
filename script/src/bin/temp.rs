use hex_literal::hex;

pub type HashElement = [u8; 32];
use lido_derisk_lib::hashing::merkleization::MerkleTreeBuilder;
use lido_derisk_lib::hashing::sha256;
use lido_derisk_lib::hashing::utils::lift_u64;
use lido_derisk_lib::ssz::validator::Validator;
use lido_derisk_lib::utils::{clone_into_array, pad_zeroes};

use log::debug;
const FAR_FUTURE_EPOCH: u64 = u64::MAX;

fn make_validator() -> Validator {
    let idx: u32 = 1;
    let pubkey: [u8; 48] = pad_zeroes(&idx.to_le_bytes());
    let pubkey_low: HashElement = clone_into_array(&pubkey[0..32]);
    let pubkey_high: [u8; 16] = clone_into_array(&pubkey[32..48]);
    let pubkey_hash = sha256::hash(&pubkey_low, &pad_zeroes(&pubkey_high));
    let withdrawal_credentials =
        hex!("0101010101010101010101010101010101010101010101010101010101010101");
    Validator {
        pubkey: pubkey_hash.into(),
        withdrawal_credentials: withdrawal_credentials.into(),
        effective_balance: 32 * 10_u64.pow(9),
        slashed: false,
        activation_eligibility_epoch: 1,
        activation_epoch: 2,
        exit_epoch: FAR_FUTURE_EPOCH,
        withdrawable_epoch: 3,
    }
}

fn plain() {
    let merkle_tree_builder = MerkleTreeBuilder::new(&sha256::hash, &sha256::ZEROHASHES);

    let values = 1_u32..=8_u32;
    println!("Values: {:?}", values);
    let to_hash: Vec<[u8; 32]> = values.map(|val| pad_zeroes(&val.to_le_bytes())).collect();

    assert_eq!(
        merkle_tree_builder.merkleize(&to_hash, Some(4), false, None),
        hex!("58a9fe130d790f2e10cfa88035af7892358d41f262b4e89ae8c295d92d3daa19")
    );
    assert_eq!(
        merkle_tree_builder.merkleize(&to_hash, Some(4), true, None),
        hex!("ce28956623cca6b6f17bba338b30d796350261275342d7bb0df7d5abd3a16ea0")
    );
    println!("Simple merkleization matches");
}

fn validator() {
    let merkle_tree_builder = MerkleTreeBuilder::new(&sha256::hash, &sha256::ZEROHASHES);
    let validator = make_validator();
    println!("Validator: {:?}", validator);

    assert_eq!(
        validator.compute_merkle_root(&merkle_tree_builder),
        hex!("f9fb29d09cc8303b2b9a5874b3f5000b90cfb2d4141f21c3ced66f0518e6e6bf"),
    );
    println!("Validator merkleization matches");
}

fn main() {
    // plain();
    validator();
}
