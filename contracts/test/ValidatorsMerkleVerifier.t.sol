// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {ValidatorsMerkleVerifier} from "../src/ValidatorsMerkleVerifier.sol";
import {SP1Verifier} from "@sp1-contracts/SP1Verifier.sol";

struct SP1ProofFixtureJson {
    bytes32 merkleRoot;
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract ValidatorsMerkleVerifierTest is Test {
    using stdJson for string;

    ValidatorsMerkleVerifier public verifier;

    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/fixtures/fixture.json");
        string memory json = vm.readFile(path);
        return (SP1ProofFixtureJson(
            json.readBytes32(".merkleRoot"),
            json.readBytes(".proof"),
            json.readBytes(".publicValues"),
            json.readBytes32(".vkey")
        ));
        // bytes memory jsonBytes = json.parseRaw(".");
        // return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();
        uint32 a = 12;
        verifier = new ValidatorsMerkleVerifier(fixture.vkey);
    }

    function test_ValidProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();
        bytes32 merkle_root = verifier.verify(
            fixture.proof,
            fixture.publicValues
        );
        assert(merkle_root == fixture.merkleRoot);
    }

    function testFail_InvalidProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        verifier.verify(fakeProof, fixture.publicValues);
    }
}
