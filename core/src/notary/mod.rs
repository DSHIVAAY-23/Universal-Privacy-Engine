//! Notary Service - ECDSA Signing for EVM Compatibility
//!
//! This module provides cryptographic signing functionality compatible with
//! Ethereum's `ecrecover` function using secp256k1 ECDSA signatures.

use ethers_core::types::{Address, Signature};
use ethers_signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::str::FromStr;
use thiserror::Error;

/// STLOP (Signed TLS-Originated Proof) structure
///
/// This represents a cryptographically signed proof of salary data
/// that can be verified on-chain by Ethereum smart contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STLOPProof {
    /// Salary amount in USD (as string to avoid precision issues)
    pub salary: String,
    
    /// Unix timestamp when the proof was generated
    pub timestamp: u64,
    
    /// ECDSA signature (65 bytes: r + s + v) in hex format
    pub signature: String,
    
    /// Notary's Ethereum address (public key)
    pub notary_pubkey: String,
}

/// Request payload for proof generation
#[derive(Debug, Deserialize)]
pub struct GenerateProofRequest {
    /// Employee's Ethereum address
    pub employee_address: String,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub notary_address: String,
}

/// Notary signing errors
#[derive(Debug, Error)]
pub enum NotaryError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
}

/// Notary Signer using ECDSA (secp256k1) for EVM compatibility
pub struct NotarySigner {
    wallet: LocalWallet,
}

impl NotarySigner {
    /// Create a new NotarySigner from a private key hex string
    ///
    /// # Arguments
    /// * `private_key_hex` - Private key in hex format (with or without 0x prefix)
    ///
    /// # Returns
    /// A new NotarySigner instance
    pub fn new(private_key_hex: &str) -> Result<Self, NotaryError> {
        let wallet = LocalWallet::from_str(private_key_hex)
            .map_err(|e| NotaryError::InvalidPrivateKey(e.to_string()))?;
        
        Ok(Self { wallet })
    }
    
    /// Get the notary's Ethereum address (public key)
    pub fn address(&self) -> Address {
        self.wallet.address()
    }
    
    /// Generate an STLOP proof for a given employee
    ///
    /// This function:
    /// 1. Fetches salary data (currently mocked as $75,000)
    /// 2. Creates a message hash using Keccak256(abi.encodePacked(employee, salary, timestamp))
    /// 3. Signs the hash with EIP-191 prefix
    /// 4. Returns the proof with signature
    ///
    /// # Arguments
    /// * `employee_address` - Employee's Ethereum address
    ///
    /// # Returns
    /// An STLOPProof containing salary, timestamp, signature, and notary address
    pub async fn generate_proof(
        &self,
        employee_address: &str,
    ) -> Result<STLOPProof, NotaryError> {
        // Parse employee address
        let employee_addr = Address::from_str(employee_address)
            .map_err(|e| NotaryError::InvalidAddress(e.to_string()))?;
        
        // TODO(phase-2): replace this with a real TLSNotary MPC proof.
        // Flow: browser opens TLS session with payroll provider → generates
        // local transcript proof → notary verifies proof via MPC handshake
        // → signs only after proof checks out. See docs/ARCHITECTURE.md.
        let salary = "75000"; // mock — $75,000
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create message hash matching Solidity's keccak256(abi.encodePacked(employee, salary, timestamp))
        let message_hash = self.create_message_hash(employee_addr, salary, timestamp);
        
        // Sign with EIP-191 prefix (same as ethers.js wallet.signMessage)
        let signature = self.wallet
            .sign_message(&message_hash)
            .await
            .map_err(|e| NotaryError::SigningFailed(e.to_string()))?;
        
        Ok(STLOPProof {
            salary: salary.to_string(),
            timestamp,
            signature: format!("0x{}", hex::encode(signature.to_vec())),
            notary_pubkey: format!("0x{:x}", self.address()),
        })
    }
    
    /// Create the message hash that matches the Solidity contract's verification logic
    ///
    /// Solidity: keccak256(abi.encodePacked(msg.sender, salary, timestamp))
    fn create_message_hash(&self, employee: Address, salary: &str, timestamp: u64) -> [u8; 32] {
        let salary_u256: u128 = salary.parse().unwrap_or(0) as u128;
        
        // abi.encodePacked format:
        // - address (20 bytes)
        // - uint256 salary (32 bytes)
        // - uint256 timestamp (32 bytes)
        let mut data = Vec::new();
        data.extend_from_slice(employee.as_bytes()); // 20 bytes
        data.extend_from_slice(&[0u8; 16]); // Pad salary to 32 bytes
        data.extend_from_slice(&salary_u256.to_be_bytes()); // 16 bytes (u128)
        data.extend_from_slice(&[0u8; 24]); // Pad timestamp to 32 bytes
        data.extend_from_slice(&timestamp.to_be_bytes()); // 8 bytes (u64)
        
        // Keccak256 hash
        let mut hasher = Keccak256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_notary_signer_creation() {
        let test_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let signer = NotarySigner::new(test_key).unwrap();
        
        // Verify the address matches the expected address for this private key
        let expected_address = "0xFCAd0B19bB29D4674531d6f115237E16AfCE377c";
        assert_eq!(
            format!("0x{:x}", signer.address()),
            expected_address
        );
    }
    
    #[tokio::test]
    async fn test_proof_generation() {
        let test_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let signer = NotarySigner::new(test_key).unwrap();
        
        let employee = "0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7";
        let proof = signer.generate_proof(employee).await.unwrap();
        
        assert_eq!(proof.salary, "75000");
        assert!(proof.signature.starts_with("0x"));
        assert_eq!(proof.signature.len(), 132); // 0x + 130 hex chars (65 bytes)
        assert!(proof.notary_pubkey.starts_with("0x"));
    }
}
