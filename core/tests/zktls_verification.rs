use universal_privacy_engine_core::data_source::{RecordedTlsProof, ZkTlsError};
use serde_json::Value;
use std::path::PathBuf;

fn load_fixtures() -> (RecordedTlsProof, Vec<u8>, Vec<u8>) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    // Navigate from core/tests/ to fixtures/zktls/
    // Path is relative to the package root during 'cargo test', so 'core/'
    let fixtures_path = PathBuf::from(manifest_dir)
        .parent() // workspace root
        .unwrap()
        .join("fixtures/zktls");

    let metadata_bytes = std::fs::read(fixtures_path.join("metadata.json")).unwrap();
    let cert_bytes = std::fs::read(fixtures_path.join("cert_chain.pem")).unwrap();
    let response_bytes = std::fs::read(fixtures_path.join("response_body.json")).unwrap();

    let metadata: Value = serde_json::from_slice(&metadata_bytes).unwrap();
    
    let proof = RecordedTlsProof {
        domain: metadata["domain"].as_str().unwrap().to_string(),
        timestamp: metadata["timestamp"].as_u64().unwrap(),
        response_hash: metadata["response_sha256"].as_str().unwrap().to_string(),
        cert_chain_hash: metadata["cert_chain_sha256"].as_str().unwrap().to_string(),
    };

    (proof, cert_bytes, response_bytes)
}

#[test]
fn valid_recorded_tls_proof_passes() {
    let (proof, cert_bytes, response_bytes) = load_fixtures();
    
    // Use a time shortly after the fixture timestamp
    // Fixture timestamp is 1735128000
    let current_time = proof.timestamp + 100; 
    let max_age = 3600;

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        current_time,
        max_age
    );

    assert!(result.is_ok());
}

#[test]
fn modified_response_hash_fails() {
    let (proof, cert_bytes, mut response_bytes) = load_fixtures();
    
    // Tamper with the response
    if let Some(last) = response_bytes.last_mut() {
        *last ^= 0xFF; 
    }

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    assert!(matches!(result, Err(ZkTlsError::ResponseTampered)));
}

#[test]
fn modified_cert_hash_fails() {
     let (proof, mut cert_bytes, response_bytes) = load_fixtures();
    
    // Tamper with the cert
    if let Some(last) = cert_bytes.last_mut() {
        *last ^= 0xFF; 
    }

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    assert!(matches!(result, Err(ZkTlsError::CertChainMismatch)));
}

#[test]
fn wrong_domain_fails() {
    let (proof, cert_bytes, response_bytes) = load_fixtures();

    let result = proof.verify(
        "evil.com", // Expecting evil.com, but proof is for example.com
        &cert_bytes,
        &response_bytes,
        proof.timestamp + 100,
        3600
    );

    match result {
        Err(ZkTlsError::DomainMismatch { expected, got }) => {
            assert_eq!(expected, "evil.com");
            assert_eq!(got, "example.com");
        }
        _ => panic!("Expected DomainMismatch"),
    }
}

#[test]
fn expired_timestamp_fails() {
    let (proof, cert_bytes, response_bytes) = load_fixtures();

    // Time is way past max age
    let max_age = 3600;
    let current_time = proof.timestamp + max_age + 1;

    let result = proof.verify(
        "example.com",
        &cert_bytes,
        &response_bytes,
        current_time,
        max_age
    );

    match result {
        Err(ZkTlsError::ReplayDetected { .. }) => {}
        _ => panic!("Expected ReplayDetected"),
    }
}
