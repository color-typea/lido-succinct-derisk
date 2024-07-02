use crate::hashing::types::HashElement;

use crate::utils::pad_zeroes;

pub fn lift_u64(value: u64) -> HashElement {
    pad_zeroes(&value.to_le_bytes())
}

pub fn pack_u64(input: &[u64; 4]) -> HashElement {
    let mut result = [0u8; 32];

    result[0..8].copy_from_slice(&input[0].to_le_bytes());
    result[8..16].copy_from_slice(&input[1].to_le_bytes());
    result[16..24].copy_from_slice(&input[2].to_le_bytes());
    result[24..32].copy_from_slice(&input[3].to_le_bytes());

    result
}
