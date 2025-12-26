use universal_privacy_engine_core::data_source::{RecordedTlsProof, ZkTlsError};
use serde_json::Value;
use std::path::PathBuf;

// Helper to manually construct a valid proof, since 'capture_zktls' isn't available in unit tests easily
// In a real scenario, we'd run the binary, but for unit tests, we'll implement a mini-signer here.
fn create_test_fixture() -> (RecordedTlsProof, Vec<u8>, Vec<u8>) {
    use ed25519_dalek::{Signer, SigningKey};
    use rand::{rngs::OsRng, RngCore};
    use sha2::{Digest, Sha256};

    let mut csprng = OsRng;
    let mut bytes = [0u8; 32];
    csprng.fill_bytes(&mut bytes);
    let signing_key = SigningKey::from_bytes(&bytes);
    let verifying_key = signing_key.verifying_key();

    let domain = "example.com";
    let timestamp = 1735128000;
    let response_body = b"{\"data\": \"test\"}";
    let cert_chain = b"MOCK_CERT";

    // Hash data
    let mut h1 = Sha256::new(); h1.update(response_body);
    let response_hash = hex::encode(h1.finalize());
    
    let mut h2 = Sha256::new(); h2.update(cert_chain);
    let cert_chain_hash = hex::encode(h2.finalize());

    // Sign canonical message
    let message = format!("{}:{}:{}:{}", domain, timestamp, response_hash, cert_chain_hash);
    let signature = signing_key.sign(message.as_bytes());

    let proof = RecordedTlsProof {
        domain: domain.to_string(),
        timestamp,
        response_hash,
        cert_chain_hash,
        notary_pubkey: hex::encode(verifying_key.to_bytes()),
        signature: hex::encode(signature.to_bytes()),
    };

    (proof, cert_chain.to_vec(), response_body.to_vec())
}

#[test]
fn valid_signed_proof_passes() {
    let (proof, cert_bytes, response_bytes) = create_test_fixture();
    
    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    assert!(result.is_ok());
}

#[test]
fn modified_response_signature_fails() {
    let (proof, cert_bytes, mut response_bytes) = create_test_fixture();
    
    // Tamper with data
    if let Some(last) = response_bytes.last_mut() { *last ^= 0xFF; }

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    // Should fail hash check first
    assert!(matches!(result, Err(ZkTlsError::ResponseTampered)));
}

#[test]
fn signature_tamper_fails() {
    let (mut proof, cert_bytes, response_bytes) = create_test_fixture();
    
    // Break the signature
    let mut sig_bytes = hex::decode(&proof.signature).unwrap();
    sig_bytes[0] ^= 0xFF; // Flip first byte
    proof.signature = hex::encode(sig_bytes);

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    assert!(matches!(result, Err(ZkTlsError::SignatureInvalid(_))));
}

#[test]
fn notary_key_spoof_fails() {
    // Generate valid proof, but verify with DIFFERENT notary public key (simulated by modifying the proof's pubkey field)
    let (mut proof, cert_bytes, response_bytes) = create_test_fixture();
    
    // Generate a different key
    use ed25519_dalek::{SigningKey, Signer};
    use rand::{rngs::OsRng, RngCore};
    let mut csprng = OsRng;
    let mut bytes = [0u8; 32];
    csprng.fill_bytes(&mut bytes);
    let new_key = SigningKey::from_bytes(&bytes);
    proof.notary_pubkey = hex::encode(new_key.verifying_key().to_bytes());

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    // Signature won't match the new pubkey
    assert!(matches!(result, Err(ZkTlsError::SignatureInvalid(_))));
}
