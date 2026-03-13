// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IVerifier
/// @notice Universal interface for ZK proof verification on zkSync Era
/// @dev Implement this interface in chain-specific verifier contracts
interface IVerifier {
    /// @notice Emitted when a proof is successfully verified
    /// @param nullifier The unique nullifier derived from the proof
    /// @param claimHash Hash of the verified claim payload
    event ProofVerified(bytes32 indexed nullifier, bytes32 indexed claimHash);

    /// @notice Emitted when a proof verification fails
    /// @param reason Human-readable reason for failure
    event ProofRejected(string reason);

    /// @notice Verifies a Groth16 ZK-SNARK proof
    /// @param proof   The encoded proof bytes (a, b, c points)
    /// @param pubSignals Public signals / public inputs array
    /// @return verified True if the proof is valid
    function verify(
        bytes calldata proof,
        uint256[] calldata pubSignals
    ) external returns (bool verified);

    /// @notice Returns the verification key commitment (for on-chain auditing)
    function verificationKeyHash() external view returns (bytes32);
}
