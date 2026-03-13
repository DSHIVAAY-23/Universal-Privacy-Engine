// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./Verifier.sol";

/**
 * @title RWAOracle
 * @notice Stores and verifies cross-chain Private RWA Storage Proofs using Oasis Sapphire's TEE.
 *
 * @dev TRUST MODEL: State variable confidentiality relies on Oasis Sapphire ParaTime.
 *      Deploy ONLY to Sapphire — on standard EVM chains (Ethereum, Polygon, etc.)
 *      the `nullifiers` mapping could theoretically be analyzed.
 *
 * @dev PROOF SCHEME: Groth16 SNARK proof over BN254.
 *      Public inputs: [stateRoot, minRequiredValue, nullifierHash]
 */
contract RWAOracle {
    // ── Config ────────────────────────────────────────────────────────────────

    address public immutable ATTESTER;
    Groth16Verifier public verifier;

    // ── Encrypted state (Sapphire ParaTime encrypts these automatically) ──────

    /// Tracks spent nullifiers to prevent double-spending of the same RWA collateral.
    mapping(uint256 => bool) private nullifiers;

    /// Stores total approved collateral per address
    mapping(address => uint256) private activeCollateral;

    // ── Events ────────────────────────────────────────────────────────────────

    /**
     * @notice Emitted when an RWA proof is successfully verified.
     * @param account        The account depositing the RWA proof.
     * @param assetContract  The source chain tokenized asset contract.
     * @param minCollateral  The verified minimum collateral amount in USD.
     */
    event RWAProofSubmitted(
        address indexed account,
        address indexed assetContract,
        uint256 minCollateral
    );

    // ── Constructor ───────────────────────────────────────────────────────────

    constructor(address _attester) {
        require(_attester != address(0), "zero attester address");
        ATTESTER = _attester;
        verifier = new Groth16Verifier();
    }

    // ── External functions ────────────────────────────────────────────────────

    /**
     * @notice Verify a ZK SNARK proof and store the nullifier to prevent replays.
     *
     * @param proof          Groth16 proof array [a0, a1, b0, b1, b2, b3, c0, c1].
     * @param nullifierHash  Deterministic hash output by the ZK circuit.
     *                       Must be globally unique to prevent double-spending.
     * @param minCollateral  The required collateral verified by the ZK range check.
     * @param assetContract  The source chain address of the RWA asset.
     */
    function submitRWAProof(
        uint256[8] calldata proof,
        uint256 nullifierHash,
        uint256 minCollateral,
        address assetContract
    ) external {
        // 1. Double-spend protection via Nullifier check
        require(!nullifiers[nullifierHash], "proof already submitted (nullifier spent)");

        // 2. REAL PROOF VERIFICATION
        uint256[2] memory a = [proof[0], proof[1]];
        uint256[2][2] memory b = [[proof[2], proof[3]], [proof[4], proof[5]]];
        uint256[2] memory c = [proof[6], proof[7]];
        
        // Hardcoding stateRoot to 0 for demo purposes. 
        // In a real production environment, stateRoot would be provided and verified on-chain against an Oracle/Bridge.
        uint256[3] memory publicSignals = [uint256(0), minCollateral, nullifierHash];

        require(verifier.verifyProof(a, b, c, publicSignals), "ZK Proof Invalid");

        // 3. Persist state
        nullifiers[nullifierHash] = true;
        activeCollateral[msg.sender] += minCollateral;

        emit RWAProofSubmitted(msg.sender, assetContract, minCollateral);
    }

    /**
     * @notice Read the caller's own verified collateral.
     * @dev Only the owner (msg.sender) can decrypt this on Sapphire.
     */
    function getActiveCollateral() external view returns (uint256) {
        return activeCollateral[msg.sender];
    }
}

