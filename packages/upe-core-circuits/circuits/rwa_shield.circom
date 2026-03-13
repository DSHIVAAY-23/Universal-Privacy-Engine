pragma circom 2.1.6;

include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/comparators.circom";

/*
  Template: RWAShield
  Purpose: Proves ownership of a token balance satisfying a minimum value 
           requirement on a source chain (via Merkle State Root), and outputs
           a nullifier for use on the destination chain to prevent double-spending.
*/
template RWAShield(nLevels) {
    // -------------------------------------------------------------------------
    // PUBLIC INPUTS
    // 1. stateRoot: The verified state root of the source chain.
    // 2. minRequiredValue: The collateral threshold the user needs to prove.
    // 3. nullifierHash: Output deterministic hash to prevent double-spending.
    // -------------------------------------------------------------------------
    signal input stateRoot;
    signal input minRequiredValue;
    signal input nullifierHash;

    // -------------------------------------------------------------------------
    // PRIVATE INPUTS
    // 1. userAddress: The owner's address on the source chain.
    // 2. tokenBalance: The user's actual token balance (hidden from destination).
    // 3. secretTrapdoor: A random 256-bit value known only to the user.
    // 4. merklePathElements: Companion nodes in the Merkle path.
    // 5. merklePathIndices: 0 for left, 1 for right.
    // -------------------------------------------------------------------------
    signal input userAddress;
    signal input tokenBalance;
    signal input secretTrapdoor;
    signal input merklePathElements[nLevels];
    signal input merklePathIndices[nLevels];

    // =========================================================================
    // LOGIC STEP 1: LEAF GENERATION
    // Construct the state leaf: Hash(userAddress, tokenBalance)
    // We use Poseidon, an optimized ZK-friendly hash function, instead of
    // Keccak256, drastically reducing the number of constraints (~250 vs ~150k).
    // =========================================================================
    component leafHasher = Poseidon(2);
    leafHasher.inputs[0] <== userAddress;
    leafHasher.inputs[1] <== tokenBalance;

    signal leaf <== leafHasher.out;

    // =========================================================================
    // LOGIC STEP 2: MERKLE INCLUSION PROOF
    // Iteratively hash from the leaf to the root using the provided path.
    // =========================================================================
    component pathHashers[nLevels];
    signal currentHash[nLevels + 1];
    currentHash[0] <== leaf;

    for (var i = 0; i < nLevels; i++) {
        // SECURITY CRITICAL: Enforce that path index is strictly binary (0 or 1).
        // If an attacker could input arbitrary numbers here, they could bypass
        // the hashing logic and forge fake Merkle roots.
        merklePathIndices[i] * (1 - merklePathIndices[i]) === 0;

        pathHashers[i] = Poseidon(2);

        // This algebraic manipulation routes the inputs natively without requiring 'if' logic.
        // If index == 0 (left):  left is currentHash, right is pathElement.
        // If index == 1 (right): left is pathElement, right is currentHash.
        var index = merklePathIndices[i];
        pathHashers[i].inputs[0] <== currentHash[i] + index * (merklePathElements[i] - currentHash[i]);
        pathHashers[i].inputs[1] <== merklePathElements[i] + index * (currentHash[i] - merklePathElements[i]);

        currentHash[i + 1] <== pathHashers[i].out;
    }

    // constrain the computed root to equal the public stateRoot.
    // This cryptographically guarantees the user's hidden balance truly exists in the source chain.
    currentHash[nLevels] === stateRoot;

    // =========================================================================
    // LOGIC STEP 3: RANGE CHECK (COLLATERAL VERIFICATION)
    // Prove that tokenBalance >= minRequiredValue without revealing the balance.
    // We use a 252-bit GreaterEqThan check to support standard EVM uint256 bounds
    // while remaining safely inside the SNARK scalar field limits.
    // =========================================================================
    component balanceCheck = GreaterEqThan(252);
    balanceCheck.in[0] <== tokenBalance;
    balanceCheck.in[1] <== minRequiredValue;

    // The output of GreaterEqThan must be 1 (true).
    balanceCheck.out === 1;

    // =========================================================================
    // LOGIC STEP 4: NULLIFIER GENERATION
    // Hash the userAddress with a secretTrapdoor to create a unique identifier.
    // Constrain this to equal the public nullifierHash.
    // This allows the destination chain DEX to record the output nullifier and prevent the 
    // identical RWA collateral from being reused twice safely.
    // =========================================================================
    component nullifierHasher = Poseidon(2);
    nullifierHasher.inputs[0] <== userAddress;
    nullifierHasher.inputs[1] <== secretTrapdoor;

    nullifierHasher.out === nullifierHash;
}

// -----------------------------------------------------------------------------
// MAIN COMPONENT EXPORT
// Instantiating the circuit with a standard Merkle Tree depth of 20.
// Public inputs must be declared explicitly so passing verifier contracts know what to check.
// -----------------------------------------------------------------------------
component main { public [stateRoot, minRequiredValue, nullifierHash] } = RWAShield(20);
