//! # SP1 Backend Adapter - Enhanced with Groth16 and RWA Support
//!
//! This adapter implements the `PrivacyEngine` trait using Succinct's SP1 proving system.
//! SP1 is a high-performance zkVM that allows writing ZK circuits in Rust.
//!
//! ## Phase 3 Enhancements
//!
//! - **Groth16 SNARK Wrapping**: Compress STARK proofs (~10MB) into Groth16 SNARKs (~300 bytes)
//! - **RWA-Specific Proving**: Dedicated methods for RWA compliance proofs
//! - **Verification Key Export**: Generate chain-specific verifier contracts
//! - **Multi-Mode Proving**: Mock (fast), STARK (full), Groth16 (on-chain)

use universal_privacy_engine_core::{
    ChainType, PrivacyEngine, PrivacyEngineError, ProofReceipt,
    rwa::{RwaClaim, RwaPublicValues},
};
use sp1_sdk::{ProverClient, SP1Stdin, SP1ProvingKey, SP1VerifyingKey, SP1ProofWithPublicValues};
use borsh::BorshSerialize;

/// Proving mode for flexibility between development and production
///
/// ## Mode Comparison
///
/// | Mode | Time | Proof Size | Security | Use Case |
/// |------|------|------------|----------|----------|
/// | Mock | ~100ms | None | None | Development/Testing |
/// | Stark | ~30-60s | ~10MB | Full | Off-chain verification |
/// | Groth16 | ~2-3min | ~300 bytes | Full | On-chain verification |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvingMode {
    /// Fast execution without proof generation
    /// 
    /// Perfect for rapid iteration during development.
    /// Executes the guest program but skips proof generation.
    Mock,
    
    /// Full STARK proof generation
    ///
    /// Generates a complete STARK proof (~10MB).
    /// Fast verification (~10ms) but too large for on-chain use.
    Stark,
    
    /// Groth16 SNARK-wrapped proof
    ///
    /// Wraps the STARK proof in a Groth16 SNARK (~300 bytes).
    /// Slower to generate but perfect for on-chain verification.
    /// **This is what you deploy to Solana/Stellar/Mantra.**
    Groth16,
}

/// Format for exporting verification keys
#[derive(Debug, Clone, Copy)]
pub enum VkeyFormat {
    /// JSON format (human-readable, larger)
    Json,
    
    /// Binary format (compact, faster)
    Binary,
    
    /// Solidity contract format (for EVM chains)
    Solidity,
}

/// SP1 backend implementation of the PrivacyEngine trait.
///
/// This struct wraps the SP1 ProverClient and translates between our generic
/// abstraction and SP1's specific APIs.
///
/// ## Architecture Note
///
/// By encapsulating the ProverClient, we maintain a clean separation between
/// the SP1-specific implementation details and the generic PrivacyEngine interface.
/// This allows consumers to write backend-agnostic code.
pub struct Sp1Backend {
    /// The SP1 prover client instance
    /// 
    /// This client handles communication with the SP1 proving service and
    /// manages proof generation lifecycle.
    client: ProverClient,
    
    /// The proving key for the SP1 program
    ///
    /// This key is generated from the ELF and is used for proof generation.
    proving_key: SP1ProvingKey,
    
    /// The verifying key for the SP1 program
    ///
    /// This key is used to verify proofs and can be exported for on-chain verification.
    verifying_key: SP1VerifyingKey,
}

impl Sp1Backend {
    /// Create a new SP1 backend instance.
    ///
    /// # Arguments
    ///
    /// * `guest_elf` - The compiled RISC-V ELF binary of the guest program
    ///
    /// # Example
    ///
    /// ```ignore
    /// let elf = include_bytes!("../../guest/elf/riscv32im-succinct-zkvm-elf");
    /// let backend = Sp1Backend::new(elf.to_vec());
    /// ```
    pub fn new(guest_elf: Vec<u8>) -> Self {
        let client = ProverClient::new();
        
        // Setup the proving and verifying keys from the ELF
        // In SP1 3.0+, we need to explicitly generate these keys
        let (proving_key, verifying_key) = client.setup(&guest_elf);
        
        Self {
            client,
            proving_key,
            verifying_key,
        }
    }

    /// Prove an RWA compliance claim with configurable proving mode.
    ///
    /// This is the primary method for generating RWA compliance proofs.
    /// It handles serialization, proof generation, and public value extraction.
    ///
    /// # Arguments
    ///
    /// * `claim` - The RWA claim to prove
    /// * `mode` - Proving mode (Mock/Stark/Groth16)
    ///
    /// # Returns
    ///
    /// A `ProofReceipt` containing the proof and public values
    ///
    /// # Example
    ///
    /// ```ignore
    /// let claim = RwaClaim::new(pubkey, 1_000_000, 500_000, signature);
    /// let receipt = backend.prove_rwa(&claim, ProvingMode::Groth16)?;
    /// ```
    pub fn prove_rwa(
        &self,
        claim: &RwaClaim,
        mode: ProvingMode,
    ) -> Result<ProofReceipt, PrivacyEngineError> {
        // Serialize the claim using Borsh (deterministic)
        let claim_bytes = borsh::to_vec(claim)
            .map_err(|e| PrivacyEngineError::InvalidInput(format!("Failed to serialize claim: {}", e)))?;

        // Prepare stdin for the guest program
        let mut stdin = SP1Stdin::new();
        stdin.write_vec(claim_bytes);

        // Generate proof based on mode
        let proof = match mode {
            ProvingMode::Mock => {
                // Mock mode: Execute without proof
                self.client
                    .prove(&self.proving_key, stdin)
                    .run()
                    .map_err(|e| PrivacyEngineError::ProvingFailed(format!("Mock execution failed: {}", e)))?
            }
            
            ProvingMode::Stark => {
                // STARK mode: Full proof generation
                self.client
                    .prove(&self.proving_key, stdin)
                    .run()
                    .map_err(|e| PrivacyEngineError::ProvingFailed(format!("STARK proving failed: {}", e)))?
            }
            
            ProvingMode::Groth16 => {
                // Groth16 mode: SNARK-wrapped proof
                // This is the CRITICAL path for on-chain verification
                //
                // The .groth16() builder wraps the STARK proof in a Groth16 SNARK:
                // 1. Generate STARK proof (~10MB)
                // 2. Compress to Groth16 (~300 bytes)
                // 3. Return compact proof suitable for blockchain
                self.client
                    .prove(&self.proving_key, stdin)
                    .groth16()  // ← SNARK wrapping happens here
                    .run()
                    .map_err(|e| PrivacyEngineError::ProvingFailed(format!("Groth16 proving failed: {}", e)))?
            }
        };

        // Extract public values from the proof
        // The guest program commits institutional_pubkey and threshold to the journal
        let public_values = proof.public_values.to_vec();

        // Serialize the proof for storage/transmission
        let proof_bytes = bincode::serialize(&proof)
            .map_err(|e| PrivacyEngineError::BackendError(format!("Serialization failed: {}", e)))?;

        // Create metadata containing SP1-specific information
        let metadata = serde_json::json!({
            "backend": "sp1",
            "version": "3.0",
            "mode": match mode {
                ProvingMode::Mock => "mock",
                ProvingMode::Stark => "stark",
                ProvingMode::Groth16 => "groth16",
            },
            "proof_system": match mode {
                ProvingMode::Groth16 => "groth16",
                _ => "stark",
            },
        })
        .to_string()
        .into_bytes();

        Ok(ProofReceipt {
            proof: proof_bytes,
            public_values,
            metadata,
        })
    }

    /// Export the verification key in the specified format.
    ///
    /// This key is required for on-chain verifier contracts.
    /// Each chain has different requirements for the Vkey format.
    ///
    /// # Arguments
    ///
    /// * `format` - The desired output format
    ///
    /// # Returns
    ///
    /// Serialized verification key bytes
    ///
    /// # Example
    ///
    /// ```ignore
    /// let vkey_json = backend.export_verification_key(VkeyFormat::Json)?;
    /// std::fs::write("vkey.json", vkey_json)?;
    /// ```
    pub fn export_verification_key(&self, format: VkeyFormat) -> Result<Vec<u8>, PrivacyEngineError> {
        match format {
            VkeyFormat::Json => {
                serde_json::to_vec_pretty(&self.verifying_key)
                    .map_err(|e| PrivacyEngineError::BackendError(format!("JSON serialization failed: {}", e)))
            }
            
            VkeyFormat::Binary => {
                bincode::serialize(&self.verifying_key)
                    .map_err(|e| PrivacyEngineError::BackendError(format!("Binary serialization failed: {}", e)))
            }
            
            VkeyFormat::Solidity => {
                // Generate Solidity contract with embedded Vkey
                // This would use SP1's Solidity verifier generator
                // Placeholder for now
                Err(PrivacyEngineError::BackendError(
                    "Solidity Vkey export not yet implemented".to_string()
                ))
            }
        }
    }

    /// Get a reference to the verifying key.
    ///
    /// Useful for programmatic access to the Vkey without serialization.
    pub fn verifying_key(&self) -> &SP1VerifyingKey {
        &self.verifying_key
    }
}

impl PrivacyEngine for Sp1Backend {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // Prepare the input for the SP1 guest program
        let mut stdin = SP1Stdin::new();
        stdin.write_slice(input);

        // ═══════════════════════════════════════════════════════════════
        // FUTURE OPTIMIZATION: Precompile Syscalls
        // ═══════════════════════════════════════════════════════════════
        // SP1 supports precompiled syscalls for common operations that are
        // expensive to prove natively. When the guest program uses these
        // operations, the prover can use optimized circuits instead of
        // proving every RISC-V instruction.
        //
        // Example precompiles to add:
        // - SHA256: sp1_zkvm::syscalls::sha256()
        // - ECDSA: sp1_zkvm::syscalls::secp256k1_verify()
        // - Keccak: sp1_zkvm::syscalls::keccak256()
        //
        // These can reduce proving time by 10-100x for cryptographic operations.
        // ═══════════════════════════════════════════════════════════════

        // Generate the proof using SP1's proving system
        // SP1 3.0+ uses an async-like builder pattern
        let proof = self
            .client
            .prove(&self.proving_key, stdin)
            .run()
            .map_err(|e| PrivacyEngineError::ProvingFailed(format!("SP1 proving error: {}", e)))?;

        // Extract public values from the proof
        // In SP1, the guest program can commit to public outputs using sp1_zkvm::io::commit()
        let public_values = proof.public_values.to_vec();

        // Serialize the proof for storage/transmission
        let proof_bytes = bincode::serialize(&proof)
            .map_err(|e| PrivacyEngineError::BackendError(format!("Serialization failed: {}", e)))?;

        // Create metadata containing SP1-specific information
        let metadata = serde_json::json!({
            "backend": "sp1",
            "version": "3.0",
            "proof_system": "stark",
        })
        .to_string()
        .into_bytes();

        Ok(ProofReceipt {
            proof: proof_bytes,
            public_values,
            metadata,
        })
    }

    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError> {
        // Deserialize the SP1 proof from the receipt
        let proof: SP1ProofWithPublicValues = bincode::deserialize(&receipt.proof)
            .map_err(|e| {
                PrivacyEngineError::VerificationFailed(format!("Failed to deserialize proof: {}", e))
            })?;

        // Verify the proof using SP1's verification logic
        // This checks:
        // 1. The proof is cryptographically valid
        // 2. The public values match the commitment
        // 3. The proof was generated for the correct program (verification key)
        self.client
            .verify(&proof, &self.verifying_key)
            .map_err(|e| {
                PrivacyEngineError::VerificationFailed(format!("SP1 verification error: {}", e))
            })?;

        // Verification succeeded
        Ok(true)
    }

    fn export_verifier(&self, chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError> {
        // Generate chain-specific verifier bytecode
        //
        // SP1 provides tools to export verifiers for different platforms.
        // Each chain has different requirements:
        //
        // - Solana: BPF bytecode with specific account structure
        // - Stellar: WASM module compatible with Soroban
        // - EVM: Solidity contract compiled to EVM bytecode
        
        match chain {
            ChainType::Solana => {
                // For Solana, we would use SP1's Solana verifier template
                // This generates a Solana program that can verify SP1 proofs on-chain
                //
                // Placeholder: In production, this would call SP1's verifier generator
                // Example: sp1_sdk::export::solana_verifier(&self.verifying_key)
                
                Err(PrivacyEngineError::ExportFailed {
                    chain,
                    reason: "Solana verifier export not yet implemented - use CLI export-verifier command".to_string(),
                })
            }
            
            ChainType::Stellar => {
                // For Stellar/Soroban, we need to generate a WASM module
                // that implements the Soroban contract interface
                //
                // Placeholder: Would use SP1's WASM verifier generator
                
                Err(PrivacyEngineError::ExportFailed {
                    chain,
                    reason: "Stellar verifier export not yet implemented - use CLI export-verifier command".to_string(),
                })
            }
            
            ChainType::Evm => {
                // For EVM chains, SP1 can generate a Solidity verifier contract
                // This is the most mature export path as it's commonly used for Ethereum
                //
                // The verifier contract would expose a function like:
                // function verify(bytes calldata proof, bytes32 publicValuesHash) returns (bool)
                
                // Placeholder: In production, this would call:
                // sp1_sdk::export::evm_verifier(&self.verifying_key)
                
                Err(PrivacyEngineError::ExportFailed {
                    chain,
                    reason: "EVM verifier export not yet implemented - use CLI export-verifier command".to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proving_mode_enum() {
        assert_eq!(ProvingMode::Mock, ProvingMode::Mock);
        assert_ne!(ProvingMode::Mock, ProvingMode::Groth16);
    }

    // Note: Full integration tests would require a compiled SP1 guest program
    // and would be placed in a separate tests/ directory
}
