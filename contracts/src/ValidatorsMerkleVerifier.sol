// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SP1Verifier} from "@sp1-contracts/SP1Verifier.sol";

/// @title Fibonacci.
/// @author Succinct Labs
/// @notice This contract implements a simple example of verifying the proof of a computing a
///         fibonacci number.
contract ValidatorsMerkleVerifier is SP1Verifier {
    /// @notice The verification key.
    bytes32 public vkey;

    constructor(bytes32 _vkey) {
        vkey = _vkey;
    }

    /// @notice The entrypoint for verifying the proof of a fibonacci number.
    /// @param proof The encoded proof.
    /// @param publicValues The encoded public values.
    function verify(
        bytes memory proof,
        bytes memory publicValues
    ) public view returns (bytes32) {
        this.verifyProof(vkey, publicValues, proof);
        bytes32 validatorMerkleRootFromProof = abi.decode(
            publicValues,
            (bytes32)
        );
        require (validatorMerkleRootFromProof == getExpectedValidatorsMerkleRoot(), "Wrong validator merkle root");
        return (validatorMerkleRootFromProof);
    }

    function getExpectedValidatorsMerkleRoot() private pure returns (bytes32) {
        return hex"af4fdb0ecc01ebb4f0edd8f1f28215c496604c73ec4d2958b03017f99c9cd4c8";
    }
}
