//! # Universal Privacy Engine - Core Abstraction Layer
//!
//! This crate defines the chain-agnostic abstraction for zero-knowledge proving systems.
//! By programming against the `PrivacyEngine` trait, consumers can swap ZK backends
//! (SP1, RISC0, etc.) without changing their application logic.
//!
//! ## Architecture Philosophy
//!
//! The adapter pattern used here provides several critical benefits:
//! 1. **Backend Independence**: Swap proving systems without breaking consumer code
//! 2. **Chain Agnosticism**: Export verifiers for multiple blockchain platforms
//! 3. **Testability**: Mock implementations for unit testing without real provers
//! 4. **Future-Proofing**: New backends can be added without modifying core abstractions

use serde::{Deserialize, Serialize};
use thiserror::Error;

// RWA (Real-World Asset) compliance types
pub mod rwa;

/// Represents the target blockchain platform for verifier deployment.
///
/// This enum allows the `PrivacyEngine` to generate chain-specific verifier bytecode,
/// accounting for differences in VM architectures, gas models, and calling conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainType {
    /// Solana - High-performance blockchain with BPF VM
    Solana,
    
    /// Stellar - Smart contract platform with Soroban runtime
    Stellar,
    
    /// EVM - Ethereum Virtual Machine compatible chains
    Evm,
}

/// A serializable proof receipt containing the ZK proof and associated metadata.
///
/// This structure is designed to be:
/// - **Portable**: Can be serialized to disk or transmitted over network
/// - **Self-Contained**: Includes all data needed for verification
/// - **Extensible**: Metadata field allows for backend-specific information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofReceipt {
    /// The actual zero-knowledge proof bytes
    pub proof: Vec<u8>,
    
    /// Public inputs/outputs that were committed to in the proof
    pub public_values: Vec<u8>,
    
    /// Backend-specific metadata (e.g., verification key hash, proof system version)
    pub metadata: Vec<u8>,
}

/// Core trait defining the interface for all ZK proving backends.
///
/// ## Design Rationale
///
/// This trait abstracts over the fundamental operations of any ZK proving system:
/// 1. **Proving**: Generate a proof from input data
/// 2. **Verification**: Validate a proof's correctness
/// 3. **Export**: Generate chain-specific verifier code
///
/// By keeping this interface minimal and focused, we ensure that any ZK backend
/// can implement it without requiring extensive adapter code.
pub trait PrivacyEngine {
    /// Generate a zero-knowledge proof from the provided input data.
    ///
    /// # Arguments
    ///
    /// * `input` - Raw bytes representing the private witness data
    ///
    /// # Returns
    ///
    /// A `ProofReceipt` containing the proof, public values, and metadata
    ///
    /// # Errors
    ///
    /// Returns `PrivacyEngineError::ProvingFailed` if proof generation fails
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError>;

    /// Verify the validity of a proof receipt.
    ///
    /// # Arguments
    ///
    /// * `receipt` - The proof receipt to verify
    ///
    /// # Returns
    ///
    /// `true` if the proof is valid, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns `PrivacyEngineError::VerificationFailed` if verification process fails
    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError>;

    /// Export a verifier contract/program for the specified blockchain.
    ///
    /// This method generates chain-specific bytecode that can verify proofs on-chain.
    /// The exact format depends on the target chain:
    /// - **Solana**: BPF bytecode for a Solana program
    /// - **Stellar**: WASM for Soroban contract
    /// - **EVM**: Solidity contract bytecode
    ///
    /// # Arguments
    ///
    /// * `chain` - The target blockchain platform
    ///
    /// # Returns
    ///
    /// Raw bytecode ready for deployment to the target chain
    ///
    /// # Errors
    ///
    /// Returns `PrivacyEngineError::ExportFailed` if verifier generation fails
    fn export_verifier(&self, chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError>;
}

/// Comprehensive error types for the Privacy Engine.
///
/// Using `thiserror` provides clean error handling with automatic `Display` and `Error` implementations.
#[derive(Debug, Error)]
pub enum PrivacyEngineError {
    /// Proof generation failed
    #[error("Proving failed: {0}")]
    ProvingFailed(String),

    /// Proof verification failed
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    /// Verifier export failed
    #[error("Verifier export failed for chain {chain:?}: {reason}")]
    ExportFailed {
        chain: ChainType,
        reason: String,
    },

    /// Input data is invalid or malformed
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Backend-specific error
    #[error("Backend error: {0}")]
    BackendError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_type_serialization() {
        let chain = ChainType::Solana;
        let serialized = bincode::serialize(&chain).unwrap();
        let deserialized: ChainType = bincode::deserialize(&serialized).unwrap();
        assert_eq!(chain, deserialized);
    }

    #[test]
    fn test_proof_receipt_serialization() {
        let receipt = ProofReceipt {
            proof: vec![1, 2, 3, 4],
            public_values: vec![5, 6, 7, 8],
            metadata: vec![9, 10],
        };
        
        let serialized = bincode::serialize(&receipt).unwrap();
        let deserialized: ProofReceipt = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(receipt.proof, deserialized.proof);
        assert_eq!(receipt.public_values, deserialized.public_values);
        assert_eq!(receipt.metadata, deserialized.metadata);
    }
}
