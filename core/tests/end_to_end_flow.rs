use universal_privacy_engine_core::{
    PrivacyEngine, ProofReceipt, ProofType, ChainType, PrivacyEngineError,
    data_source::{DataProvider, ZkInputBuilder, DataError},
};
use serde_json::json;
use async_trait::async_trait;

/// Mock Provider to simulate HTTP fetching without network calls
struct MockProvider;

#[async_trait]
impl DataProvider for MockProvider {
    async fn fetch(&self, _source: &str, _query: &str) -> Result<Vec<u8>, DataError> {
        // Simulate fetching a JSON and selecting "balance" field
        // In a real scenario, this would be the result of `select_json_field`
        let response_value = 1000u64; // The extracted value
        Ok(serde_json::to_vec(&response_value).unwrap())
    }
    
    fn verify_tls_proof(&self, _proof: &universal_privacy_engine_core::data_source::RecordedTlsProof) -> Result<(), universal_privacy_engine_core::data_source::ZkTlsError> { 
        // Mocking zkTLS verification passing
        Ok(()) 
    }
}

/// Mock Backend to simulate Sp1Prover behavior
/// 
/// We use this because we don't have a compiled SP1 guest ELF in this environment.
/// This mock ensures the architectural flow is valid.
struct MockSp1Backend;

impl PrivacyEngine for MockSp1Backend {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // Simulate proof generation
        // In reality, this would run the SP1 zkVM
        Ok(ProofReceipt {
            proof_type: ProofType::ZkProof,
            proof: vec![0xCA, 0xFE, 0xBA, 0xBE], // Fake proof bytes
            public_values: input.to_vec(), // In this mock, we just echo input
            metadata: b"mock_sp1_backend".to_vec(),
        })
    }

    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError> {
        // Verify the mock proof matches our expectations
        if receipt.proof == vec![0xCA, 0xFE, 0xBA, 0xBE] {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn export_verifier(&self, _chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn test_full_system_flow() {
    // 1. Fetch structured JSON data (mocked)
    // We simulate fetching "data.balance" from "https://api.bank.com"
    let provider = MockProvider;
    let fetched_data = provider
        .fetch("https://api.bank.com/data", "data.balance")
        .await
        .expect("Failed to fetch data");

    // Verify we got the expected data (serialized 1000)
    let balance: u64 = serde_json::from_slice(&fetched_data).expect("Failed to deserialize");
    assert_eq!(balance, 1000);

    // 2. Extract a field using the JSON selector
    // (Implicitly handled by provider.fetch in this integration test structure)

    // 3. Combine public data + a secret via ZkInputBuilder
    let mut builder = ZkInputBuilder::new();
    
    // Add the public fetched data
    builder.add_public_data(fetched_data);
    
    // Add a user secret (e.g., private key)
    let secret_key = b"super_secret_key".to_vec();
    builder.add_secret(secret_key);
    
    let zk_input = builder.build();

    // 4. Generate ProofReceipt
    // Use our Mock backend that implements PrivacyEngine trait
    let backend = MockSp1Backend;
    
    println!("Generating proof for input size: {} bytes", zk_input.len());
    let receipt = backend.prove(&zk_input).expect("Proof generation failed");
    
    assert_eq!(receipt.proof_type, ProofType::ZkProof);
    assert!(!receipt.proof.is_empty());

    // 5. Verify the proof using the corresponding verifier logic
    println!("Verifying proof...");
    let is_valid = backend.verify(&receipt).expect("Verification failed");
    
    assert!(is_valid);
    println!("Integration test passed successfully!");
}
