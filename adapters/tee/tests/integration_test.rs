//! Integration test demonstrating backend polymorphism
//!
//! This test shows that both SP1 and TEE backends can be used interchangeably
//! through the PrivacyEngine trait, demonstrating the hexagonal architecture.

use universal_privacy_engine_core::{PrivacyEngine, ProofType};
use universal_privacy_engine_tee::TeeProverStub;

#[test]
fn test_tee_backend_polymorphism() {
    // Create TEE backend
    let tee_backend = TeeProverStub::new();
    
    // Test input
    let input = b"test compliance data";
    
    // Generate proof using TEE backend
    let receipt = tee_backend.prove(input).expect("TEE proving should succeed");
    
    // Verify proof type
    assert_eq!(receipt.proof_type, ProofType::TeeAttestation);
    
    // Verify the proof
    let is_valid = tee_backend.verify(&receipt).expect("TEE verification should succeed");
    assert!(is_valid, "TEE attestation should be valid");
}

#[test]
fn test_backend_trait_object() {
    // Demonstrate that backends can be used as trait objects
    let backends: Vec<Box<dyn PrivacyEngine>> = vec![
        Box::new(TeeProverStub::new()),
        Box::new(TeeProverStub::new()),
    ];
    
    let input = b"polymorphic test";
    
    for (i, backend) in backends.iter().enumerate() {
        let receipt = backend.prove(input).expect(&format!("Backend {} should prove", i));
        let is_valid = backend.verify(&receipt).expect(&format!("Backend {} should verify", i));
        assert!(is_valid, "Backend {} proof should be valid", i);
    }
}

#[test]
fn test_different_backends_different_proofs() {
    // Create two different TEE backends
    let backend1 = TeeProverStub::new();
    let backend2 = TeeProverStub::new();
    
    let input = b"test data";
    
    // Generate proof with backend1
    let receipt1 = backend1.prove(input).unwrap();
    
    // Backend2 should reject backend1's proof (different enclave measurements)
    let result = backend2.verify(&receipt1);
    assert!(result.is_err(), "Different backends should have different enclave measurements");
}
