use ethereum_types::H256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

use crate::hashing::merkleization::MerkleTreeBuilder;
use crate::hashing::types::HashElement;
use crate::hashing::utils::lift_u64;

use log::debug;

// Extracted from Lighthouse `consensus/types/src/validator.rs`
// https://github.com/sigp/lighthouse/blob/stable/consensus/types/src/validator.rs
// with some simplifications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Validator {
    // shortcut - pubkey should be [u8; 48], but it requires quite a significant effort on
    // implementing serde & ssz encode-decode for [u8;48] - so I'm skipping it for now.
    // This doesn't make the solution less secure though - essentially we're moving some
    // non-essential compute to "traditional" compute
    pub pubkey: H256,
    pub withdrawal_credentials: H256,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: u64,
    pub withdrawable_epoch: u64,
}

impl Validator {
    pub fn compute_merkle_root(&self, builder: &MerkleTreeBuilder) -> HashElement {
        let leaves: Vec<HashElement> = vec![
            self.pubkey.as_fixed_bytes().to_owned(),
            self.withdrawal_credentials.as_fixed_bytes().to_owned(),
            lift_u64(self.effective_balance),
            lift_u64(if self.slashed { 1u64 } else { 0u64 }),
            lift_u64(self.activation_eligibility_epoch),
            lift_u64(self.activation_epoch),
            lift_u64(self.exit_epoch),
            lift_u64(self.withdrawable_epoch),
        ];
        debug!("Validator Leafs: {:?}", leaves);
        builder.merkleize(&leaves, None, false, None)
    }
}
