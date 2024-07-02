//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be verified
//! on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --package fibonacci-script --bin prove --release
//! ```

use clap::Parser;
use serde::{Deserialize, Serialize};
use sp1_sdk::{NetworkProver, ProverClient, SP1VerifyingKey};
use std::path::PathBuf;

use hex_literal::hex;

use sp1_sdk::HashableKey;

pub const ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

type Proof = sp1_sdk::SP1PlonkBn254Proof;

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    #[clap(long)]
    proof_id: String,
    #[clap(long)]
    network_pk: String,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1MerkleProofFixture {
    merkle_root: String,
    public_values: String,
    proof: String,
    vkey: String,
}

fn write_fixture(expected_merkle_root: &[u8; 32], vk: &SP1VerifyingKey, proof: &Proof) {
    // Create the testing fixture so we can test things end-ot-end.
    let fixture = SP1MerkleProofFixture {
        merkle_root: hex::encode(*expected_merkle_root),
        vkey: vk.bytes32().to_string(),
        public_values: proof.public_values.bytes().to_string(),
        proof: proof.bytes().to_string(),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    //
    // Note that the verification key stays the same regardless of the input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values whicha are publically commited to by the zkVM.
    //
    // If you need to expose the inputs or outputs of your program, you should commit them in
    // the public values.
    println!("Public Values: {}", fixture.public_values);

    // The proof proves to the verifier that the program was executed with some inputs that led to
    // the give public values.
    println!("Proof Bytes: {}", fixture.proof);

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join("fixture.json"),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture")
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    let args = ProveArgs::parse();

    let network_pk_raw =
        std::fs::read_to_string(args.network_pk).expect("Cannot read SP1 network private key");

    let network_pk = network_pk_raw.trim();

    let expected_merkle_root =
        hex!("b8d9b2a2293a5215b31ed0ad22c59a7d2cbaf31a349210eeef3fe3ce23f06f70");

    // let proof_id = args.proof_id;
    let proof_id = "proofrequest_01j16a01bqe80s40kkshbjat8h";
    let network_prover = NetworkProver::new_from_key(&network_pk);
    println!("Proof ID: {}", proof_id);
    let proof: Proof = network_prover.wait_proof(&proof_id).await.unwrap();

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);

    // Verify proof
    // client
    //     .verify_plonk(&proof, &vk)
    //     // .verify_compressed(&proof, &vk)
    //     .expect("verification failed");
    // println!("Verified successfully");

    write_fixture(&expected_merkle_root, &vk, &proof);
}

// RUST_LOG=debug cargo run --bin proof_downloader --release -- --network-pk ~/.sp1/network_pk --proof-id proofrequest_01j16a01bqe80s40kkshbjat8h
// RUST_LOG=debug cargo run --bin proof_downloader --release -- --network-pk ~/.sp1/network_pk --proof-id proofrequest_01j1gnxvrcej8bj0wk3kw18skm
