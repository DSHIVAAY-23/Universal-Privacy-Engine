//! Recorded zkTLS Proof definition and verification logic.
//!
//! This module implements a deterministic verification system for recorded TLS proofs.
//! It serves as a fixture-based placeholder for full zkTLS (TLSNotary/DECO) implementation.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use sha2::{Digest, Sha256};
use crate::data_source::DataError;

/// Error types for zkTLS verification
#[derive(Debug, Error)]
pub enum ZkTlsError {
    #[error("Domain mismatch: expected {expected}, got {got}")]
    DomainMismatch { expected: String, got: String },
    
    #[error("Certificate chain mismatch")]
    CertChainMismatch,
    
    #[error("Response body hash mismatch / integrity failure")]
    ResponseTampered,
    
    #[error("Replay detected: proof timestamp {timestamp} is too old (max age {max_age}s)")]
    ReplayDetected { timestamp: u64, max_age: u64 },
}

/// A recorded proof of a TLS session.
/// 
/// Contains metadata verifying that specific data originated from a specific domain
/// at a specific time, signed by a valid certificate chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedTlsProof {
    /// The domain that served the response (e.g., "example.com")
    pub domain: String,
    
    /// UNIX timestamp (seconds) when the session occurred
    pub timestamp: u64,
    
    /// SHA256 hash of the HTTP response body
    pub response_hash: String,
    
    /// SHA256 hash of the PEM-encoded certificate chain
    pub cert_chain_hash: String,
}

impl RecordedTlsProof {
    /// Verify the recorded proof against expected values and policy.
    ///
    /// # Arguments
    ///
    /// * `expected_domain` - The domain we expect the data to come from
    /// * `cert_chain_pem` - The raw PEM bytes of the certificate chain to verify against hash
    /// * `response_body` - The raw JSON bytes we are verifying
    /// * `current_time` - Current UNIX timestamp
    /// * `max_age_secs` - Maximum allowed age of the proof in seconds
    pub fn verify(
        &self,
        expected_domain: &str,
        cert_chain_pem: &[u8],
        response_body: &[u8],
        current_time: u64,
        max_age_secs: u64,
    ) -> Result<(), ZkTlsError> {
        // 1. Verify Domain
        if self.domain != expected_domain {
            return Err(ZkTlsError::DomainMismatch {
                expected: expected_domain.to_string(),
                got: self.domain.clone(),
            });
        }

        // 2. Verify Timestamp Freshness
        if current_time > self.timestamp + max_age_secs {
            return Err(ZkTlsError::ReplayDetected {
                timestamp: self.timestamp,
                max_age: max_age_secs,
            });
        }
        
        // 3. Verify Certificate Chain Hash
        let computed_cert_hash = sha256_hex(cert_chain_pem);
        if computed_cert_hash != self.cert_chain_hash {
            return Err(ZkTlsError::CertChainMismatch);
        }
        
        // 4. Verify Response Integrity
        let computed_response_hash = sha256_hex(response_body);
        if computed_response_hash != self.response_hash {
            return Err(ZkTlsError::ResponseTampered);
        }

        Ok(())
    }
}

/// Helper to compute SHA256 hash as hex string
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
