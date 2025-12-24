//! # RWA (Real-World Asset) Compliance Types
//!
//! This module defines the core data structures for the RWA Compliance Passport system.
//! These types are shared between the host (prover) and guest (zkVM) environments.
//!
//! ## Design Principles
//!
//! 1. **no_std Compatible**: Must work in the constrained zkVM environment
//! 2. **Borsh Serialization**: Deterministic encoding for ZK proofs
//! 3. **Fixed-Size Types**: Prefer arrays over Vec for predictable memory usage
//! 4. **Cryptographic Primitives**: Ed25519 for institutional signatures

use borsh::{BorshDeserialize, BorshSerialize};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// Support for serializing large arrays (>32 bytes) with serde
#[cfg(feature = "std")]
use serde_big_array::BigArray;

/// A claim asserting that an institution holds a certain balance of real-world assets.
///
/// This structure contains both private data (balance, signature) and public data
/// (institutional identity, threshold). The zkVM guest program will verify the claim
/// and selectively reveal only the public portions.
///
/// ## Fields
///
/// - `institutional_pubkey`: Ed25519 public key identifying the institution
/// - `balance`: Private balance amount (in smallest unit, e.g., cents or wei)
/// - `threshold`: Minimum required balance for compliance
/// - `signature`: Ed25519 signature over the balance, signed by the institution's private key
///
/// ## Signature Scheme
///
/// The signature is computed as:
/// ```text
/// signature = Ed25519.sign(private_key, balance.to_le_bytes())
/// ```
///
/// In production, the message should include additional context:
/// - Timestamp (prevent replay attacks)
/// - Nonce (prevent signature reuse)
/// - Asset type identifier (if supporting multiple assets)
///
/// ## Example
///
/// ```ignore
/// use universal_privacy_engine_core::rwa::RwaClaim;
///
/// // Institution has $1,000,000 in assets, threshold is $500,000
/// let claim = RwaClaim {
///     institutional_pubkey: institution_pubkey,
///     balance: 1_000_000_00,  // $1M in cents
///     threshold: 500_000_00,   // $500K in cents
///     signature: institution_signature,
/// };
/// ```
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RwaClaim {
    /// Ed25519 public key of the institution (32 bytes)
    ///
    /// This identifies which institution is making the claim.
    /// It will be committed to the public journal, so verifiers know
    /// which institution passed compliance.
    pub institutional_pubkey: [u8; 32],

    /// Private balance amount (u64)
    ///
    /// This is the sensitive data we want to keep private.
    /// The zkVM will verify it meets the threshold without revealing the exact amount.
    ///
    /// Units depend on the asset type:
    /// - USD: cents (e.g., 100_00 = $100.00)
    /// - BTC: satoshis (e.g., 100_000_000 = 1 BTC)
    /// - ETH: wei (e.g., 1_000_000_000_000_000_000 = 1 ETH)
    pub balance: u64,

    /// Minimum required balance for compliance (u64)
    ///
    /// This will be committed to the public journal.
    /// Verifiers will know the institution met this threshold, but not by how much.
    pub threshold: u64,

    /// Ed25519 signature over the balance (64 bytes)
    ///
    /// This proves the institution actually signed off on this balance claim.
    /// The signature is verified inside the zkVM using the Ed25519 precompile.
    ///
    /// Signature format: Ed25519(private_key, balance.to_le_bytes())
    #[cfg_attr(feature = "std", serde(with = "BigArray"))]
    pub signature: [u8; 64],
}

impl RwaClaim {
    /// Create a new RWA claim.
    ///
    /// # Arguments
    ///
    /// * `institutional_pubkey` - Ed25519 public key of the institution
    /// * `balance` - Private balance amount
    /// * `threshold` - Minimum required balance
    /// * `signature` - Ed25519 signature over the balance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let claim = RwaClaim::new(
    ///     institution_pubkey,
    ///     1_000_000_00,  // $1M
    ///     500_000_00,    // $500K threshold
    ///     signature,
    /// );
    /// ```
    pub fn new(
        institutional_pubkey: [u8; 32],
        balance: u64,
        threshold: u64,
        signature: [u8; 64],
    ) -> Self {
        Self {
            institutional_pubkey,
            balance,
            threshold,
            signature,
        }
    }

    /// Get the message that should be signed for this claim.
    ///
    /// Currently, this is just the balance encoded as little-endian bytes.
    /// In production, you should include additional context to prevent replay attacks.
    ///
    /// # Returns
    ///
    /// 8-byte array containing the balance in little-endian format
    pub fn message_to_sign(&self) -> [u8; 8] {
        self.balance.to_le_bytes()
    }
}

/// Public values committed to the journal by the guest program.
///
/// This structure represents what verifiers will see after proof generation.
/// It contains only the non-sensitive information needed for compliance verification.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RwaPublicValues {
    /// Ed25519 public key of the institution that passed compliance
    pub institutional_pubkey: [u8; 32],

    /// The threshold that was met (but not the actual balance)
    pub threshold: u64,
}

impl RwaPublicValues {
    /// Create new public values from a claim.
    ///
    /// This extracts only the public portions of an RwaClaim.
    pub fn from_claim(claim: &RwaClaim) -> Self {
        Self {
            institutional_pubkey: claim.institutional_pubkey,
            threshold: claim.threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rwa_claim_creation() {
        let claim = RwaClaim::new(
            [1u8; 32],  // Mock pubkey
            1_000_000,  // $1M balance
            500_000,    // $500K threshold
            [2u8; 64],  // Mock signature
        );

        assert_eq!(claim.balance, 1_000_000);
        assert_eq!(claim.threshold, 500_000);
    }

    #[test]
    fn test_message_to_sign() {
        let claim = RwaClaim::new([0u8; 32], 12345, 10000, [0u8; 64]);
        let message = claim.message_to_sign();

        // Verify it's the little-endian encoding of 12345
        assert_eq!(message, 12345u64.to_le_bytes());
    }

    #[test]
    fn test_borsh_serialization() {
        let claim = RwaClaim::new([1u8; 32], 1_000_000, 500_000, [2u8; 64]);

        // Serialize and deserialize
        let bytes = borsh::to_vec(&claim).unwrap();
        let deserialized = RwaClaim::try_from_slice(&bytes).unwrap();

        assert_eq!(claim.balance, deserialized.balance);
        assert_eq!(claim.threshold, deserialized.threshold);
    }

    #[test]
    fn test_public_values_extraction() {
        let claim = RwaClaim::new([1u8; 32], 1_000_000, 500_000, [2u8; 64]);
        let public_values = RwaPublicValues::from_claim(&claim);

        assert_eq!(public_values.institutional_pubkey, claim.institutional_pubkey);
        assert_eq!(public_values.threshold, claim.threshold);
    }
}
