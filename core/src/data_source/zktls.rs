use serde::{Deserialize, Serialize};
use thiserror::Error;
use sha2::{Digest, Sha256};
use ed25519_dalek::{VerifyingKey, Signature, Verifier}; // robust API


#[derive(Debug, Error)]
pub enum ZkTlsError {
    #[error("Domain mismatch: expected {expected}, got {got}")]
    DomainMismatch { expected: String, got: String },

    #[error("Certificate chain mismatch")]
    CertChainMismatch,

    #[error("Response body hash mismatch / integrity failure")]
    ResponseTampered,

    #[error("Replay detected: proof timestamp {timestamp} is too old")]
    ReplayDetected { timestamp: u64, max_age: u64 },

    #[error("Notary Signature Invalid: {0}")]
    SignatureInvalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedTlsProof {
    pub domain: String,
    pub timestamp: u64,
    pub response_hash: String,
    pub cert_chain_hash: String,

    /// The Public Key of the Notary who witnessed this session (Hex)
    pub notary_pubkey: String,

    /// The Ed25519 Signature of the Metadata (Hex)
    /// Signs: sha256(domain + ":" + timestamp + ":" + response_hash + ":" + cert_chain_hash)
    pub signature: String,
}

impl RecordedTlsProof {
    pub fn verify(
        &self,
        expected_domain: &str,
        cert_chain_pem: &[u8],
        response_body: &[u8],
        current_time: u64,
        max_age_secs: u64,
    ) -> Result<(), ZkTlsError> {
        // 0. Basic domain match
        if self.domain != expected_domain {
            return Err(ZkTlsError::DomainMismatch {
                expected: expected_domain.to_string(),
                got: self.domain.clone(),
            });
        }

        // 1. Timestamp checks: not too old
        if current_time > self.timestamp + max_age_secs {
            return Err(ZkTlsError::ReplayDetected {
                timestamp: self.timestamp,
                max_age: max_age_secs,
            });
        }

        // 1b. Optional: reject timestamps too far in the future (small clock skew)
        const MAX_FUTURE_SKEW_SECS: u64 = 300;
        if self.timestamp > current_time + MAX_FUTURE_SKEW_SECS {
            return Err(ZkTlsError::SignatureInvalid("Timestamp is in the future".into()));
        }

        // 2. Integrity checks (hashes)
        let computed_cert_hash = sha256_hex(cert_chain_pem);
        if computed_cert_hash != self.cert_chain_hash {
            return Err(ZkTlsError::CertChainMismatch);
        }

        let computed_response_hash = sha256_hex(response_body);
        if computed_response_hash != self.response_hash {
            return Err(ZkTlsError::ResponseTampered);
        }

        // 3. Signature (authenticity)
        // This ensures the metadata was produced by holder of notary_pubkey.
        // NOTE: You MUST ensure notary_pubkey is trusted (allowlist / PKI). See TODO below.
        self.verify_signature()?;

        Ok(())
    }

    fn verify_signature(&self) -> Result<(), ZkTlsError> {
        // Canonical message: "domain:timestamp:response_hash:cert_chain_hash"
        let message = format!(
            "{}:{}:{}:{}",
            self.domain, self.timestamp, self.response_hash, self.cert_chain_hash
        );
        let message_bytes = message.as_bytes();

        // Decode pubkey hex
        let pub_key_bytes = hex::decode(&self.notary_pubkey)
            .map_err(|_| ZkTlsError::SignatureInvalid("Bad hex in notary_pubkey".into()))?;
        if pub_key_bytes.len() != 32 {
            return Err(ZkTlsError::SignatureInvalid(format!(
                "Invalid public key length: {} bytes",
                pub_key_bytes.len()
            )));
        }

        // Decode signature hex
        let sig_bytes = hex::decode(&self.signature)
            .map_err(|_| ZkTlsError::SignatureInvalid("Bad hex in signature".into()))?;
        if sig_bytes.len() != 64 {
            return Err(ZkTlsError::SignatureInvalid(format!(
                "Invalid signature length: {} bytes",
                sig_bytes.len()
            )));
        }

        // Convert to fixed-size arrays then into types
        let pub_key_arr: [u8; 32] = pub_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| ZkTlsError::SignatureInvalid("Public key conversion failed".into()))?;
        let sig_arr: [u8; 64] = sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| ZkTlsError::SignatureInvalid("Signature conversion failed".into()))?;

        // ed25519-dalek 2.0 API: Use VerifyingKey instead of PublicKey
        let verifying_key = VerifyingKey::from_bytes(&pub_key_arr)
            .map_err(|e| ZkTlsError::SignatureInvalid(format!("VerifyingKey parse error: {}", e)))?;
        
        // ed25519-dalek 2.0 API: Signature::from([u8; 64])
        let signature = Signature::from(sig_arr);

        // Cryptographic verification
        verifying_key
            .verify(message_bytes, &signature)
            .map_err(|_| ZkTlsError::SignatureInvalid("Cryptographic verification failed".into()))
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
