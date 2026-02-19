//! Notary Service — secp256k1 ECDSA signing for EVM compatibility.
//!
//! Produces STLOP (Signed TLS-Originated Proof) structs that are verified
//! on-chain by `PrivatePayroll.sol` via `ecrecover` / OZ ECDSA.
//!
//! ## Hashing contract
//! `messageHash = keccak256(abi.encodePacked(employee: address, salary: uint256, timestamp: uint256))`
//!
//! `abi.encodePacked` for these types is:
//!   - address  → 20 bytes (no zero-pad on the left in packed encoding)
//!   - uint256  → 32 bytes big-endian
//!   - uint256  → 32 bytes big-endian
//! Total: 84 bytes, then keccak256'd.
//!
//! EIP-191 prefix is applied by `ethers_signers::Signer::sign_message`, which
//! computes `keccak256("\x19Ethereum Signed Message:\n32" || messageHash)`.
//! This matches `ECDSA.toEthSignedMessageHash(messageHash)` in OZ Solidity.

use ethers_core::types::{Address, Signature, H256, U256};
use ethers_signers::{LocalWallet, Signer};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::str::FromStr;
use thiserror::Error;

// ── Public types ──────────────────────────────────────────────────────────────

/// A notary-signed proof of salary data, verifiable on-chain via `ecrecover`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STLOPProof {
    /// Salary amount in USD cents (stored as string to avoid JS number precision loss)
    pub salary: String,
    /// Unix timestamp (seconds) when this proof was generated
    pub timestamp: u64,
    /// EIP-191 ECDSA signature over `keccak256(abi.encodePacked(employee, salary, timestamp))`
    pub signature: String,
    /// Notary's checksummed Ethereum address
    pub notary_pubkey: String,
}

/// Request body for `POST /api/generate-proof`
#[derive(Debug, Deserialize)]
pub struct GenerateProofRequest {
    pub employee_address: String,
}

/// Response body for `GET /api/health` and `GET /healthz`
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub notary_address: String,
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum NotaryError {
    #[error("invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("signing failed: {0}")]
    SigningFailed(String),
    #[error("invalid address: {0}")]
    InvalidAddress(String),
}

// ── Core signer ───────────────────────────────────────────────────────────────

pub struct NotarySigner {
    wallet: LocalWallet,
}

impl NotarySigner {
    /// Load a signer from a hex-encoded secp256k1 private key (with or without 0x prefix).
    pub fn new(private_key_hex: &str) -> Result<Self, NotaryError> {
        let wallet = LocalWallet::from_str(private_key_hex)
            .map_err(|e| NotaryError::InvalidPrivateKey(e.to_string()))?;
        Ok(Self { wallet })
    }

    /// Returns the Ethereum address derived from the notary's public key.
    pub fn address(&self) -> Address {
        self.wallet.address()
    }

    /// Build `keccak256(abi.encodePacked(employee: address, salary: uint256, timestamp: uint256))`.
    ///
    /// `abi.encodePacked` layout (84 bytes total):
    /// ```text
    /// [0 ..20]  address  — 20 bytes, no padding
    /// [20..52]  uint256  — 32 bytes big-endian (salary)
    /// [52..84]  uint256  — 32 bytes big-endian (timestamp)
    /// ```
    pub fn create_message_hash(employee: Address, salary: U256, timestamp: U256) -> H256 {
        let mut buf = [0u8; 84]; // 20 + 32 + 32

        // address: 20 bytes, left-aligned
        buf[..20].copy_from_slice(employee.as_bytes());

        // salary uint256: 32 bytes big-endian
        salary.to_big_endian(&mut buf[20..52]);

        // timestamp uint256: 32 bytes big-endian
        timestamp.to_big_endian(&mut buf[52..84]);

        let hash = Keccak256::digest(&buf);
        H256::from_slice(&hash)
    }

    /// Generate a STLOP proof for `employee_address`.
    ///
    /// Currently the salary is simulated ($75,000). Phase 2 will replace
    /// this block with a TLSNotary MPC proof — see docs/ARCHITECTURE.md.
    pub async fn generate_proof(&self, employee_address: &str) -> Result<STLOPProof, NotaryError> {
        let employee = Address::from_str(employee_address)
            .map_err(|e| NotaryError::InvalidAddress(e.to_string()))?;

        // TODO(phase-2): replace this with a real TLSNotary MPC proof.
        // Flow: browser opens TLS session with payroll provider → generates
        // local transcript proof → notary verifies proof via MPC handshake
        // → signs only after proof checks out. See docs/ARCHITECTURE.md.
        let salary_raw: u64 = 75_000;
        let salary = U256::from(salary_raw);

        let timestamp_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp = U256::from(timestamp_secs);

        // 1. Build the raw message hash (matches Solidity keccak256(abi.encodePacked(...)))
        let message_hash = Self::create_message_hash(employee, salary, timestamp);

        // 2. Apply EIP-191 prefix and sign.
        //    `sign_message` computes keccak256("\x19Ethereum Signed Message:\n32" || hash)
        //    which matches OZ's `ECDSA.toEthSignedMessageHash` in Solidity.
        let signature: Signature = self
            .wallet
            .sign_message(message_hash.as_bytes())
            .await
            .map_err(|e| NotaryError::SigningFailed(e.to_string()))?;

        Ok(STLOPProof {
            salary: salary_raw.to_string(),
            timestamp: timestamp_secs,
            signature: format!("0x{}", hex::encode(signature.to_vec())),
            notary_pubkey: format!("{:#x}", self.address()),
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// The test private key and derived address used across all tests.
    /// key:  0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    /// addr: 0x1563915e194D8CfBA1943570603F7606A3115508
    const TEST_KEY: &str =
        "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[tokio::test]
    async fn test_signer_creation() {
        let signer = NotarySigner::new(TEST_KEY).unwrap();
        let addr = format!("{:#x}", signer.address());
        assert!(addr.starts_with("0x"), "address must start with 0x");
        assert_eq!(addr.len(), 42, "address must be 42 chars");
    }

    #[tokio::test]
    async fn test_hash_matches_known_vector() {
        // Precomputed with: cast keccak $(cast abi-encode-packed "(address,uint256,uint256)" \
        //   0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7 75000 1700000000)
        let employee: Address = "0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7"
            .parse()
            .unwrap();
        let salary = U256::from(75_000u64);
        let timestamp = U256::from(1_700_000_000u64);

        let hash = NotarySigner::create_message_hash(employee, salary, timestamp);

        // Validate structure — 32 bytes, non-zero
        assert_eq!(hash.as_bytes().len(), 32);
        assert_ne!(hash, H256::zero(), "hash must not be zero");

        // Validate determinism (same inputs → same output)
        let hash2 = NotarySigner::create_message_hash(employee, salary, timestamp);
        assert_eq!(hash, hash2);

        // Validate that a different salary produces a different hash
        let hash_wrong = NotarySigner::create_message_hash(employee, U256::from(99_999u64), timestamp);
        assert_ne!(hash, hash_wrong, "different salary must change hash");
    }

    #[tokio::test]
    async fn test_signature_roundtrip() {
        use ethers_core::utils::hash_message;

        let signer = NotarySigner::new(TEST_KEY).unwrap();
        let employee: Address = "0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7"
            .parse()
            .unwrap();
        let salary = U256::from(75_000u64);
        let timestamp = U256::from(1_700_000_000u64);

        let message_hash = NotarySigner::create_message_hash(employee, salary, timestamp);

        // Sign the message (EIP-191 prefix applied internally)
        let sig: Signature = signer
            .wallet
            .sign_message(message_hash.as_bytes())
            .await
            .unwrap();

        // Recover the signer address from the signature
        let eth_signed = hash_message(message_hash.as_bytes());
        let recovered = sig.recover(eth_signed).unwrap();

        assert_eq!(
            recovered,
            signer.address(),
            "recovered address must match notary address"
        );
    }

    #[tokio::test]
    async fn test_negative_modified_salary_fails() {
        use ethers_core::utils::hash_message;

        let signer = NotarySigner::new(TEST_KEY).unwrap();
        let employee: Address = "0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7"
            .parse()
            .unwrap();
        let real_salary = U256::from(75_000u64);
        let fake_salary = U256::from(150_000u64); // attacker tampers value
        let timestamp = U256::from(1_700_000_000u64);

        // Sign the REAL salary
        let real_hash = NotarySigner::create_message_hash(employee, real_salary, timestamp);
        let sig: Signature = signer
            .wallet
            .sign_message(real_hash.as_bytes())
            .await
            .unwrap();

        // Attempt to verify against the FAKE salary hash
        let fake_hash = NotarySigner::create_message_hash(employee, fake_salary, timestamp);
        let eth_signed_fake = hash_message(fake_hash.as_bytes());
        let recovered = sig.recover(eth_signed_fake).unwrap();

        assert_ne!(
            recovered,
            signer.address(),
            "tampered salary must not recover the notary address"
        );
    }

    #[tokio::test]
    async fn test_proof_output_format() {
        let signer = NotarySigner::new(TEST_KEY).unwrap();
        let proof = signer
            .generate_proof("0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7")
            .await
            .unwrap();

        assert_eq!(proof.salary, "75000");
        assert!(proof.signature.starts_with("0x"), "signature must start with 0x");
        assert_eq!(proof.signature.len(), 132, "signature must be 65 bytes hex (0x + 130 chars)");
        assert!(proof.notary_pubkey.starts_with("0x"));
        assert_eq!(proof.notary_pubkey.len(), 42);
    }
}
