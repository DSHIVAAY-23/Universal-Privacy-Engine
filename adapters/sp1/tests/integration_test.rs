//! Integration tests for the Universal Privacy Engine
//!
//! These tests verify the end-to-end workflow:
//! 1. Create RWA claim
//! 2. Generate proof (mock mode for speed)
//! 3. Verify proof
//! 4. Export verification key

use borsh::BorshDeserialize;
use universal_privacy_engine_core::rwa::RwaClaim;
use universal_privacy_engine_sp1::{ProvingMode, VkeyFormat};

#[test]
fn test_rwa_claim_creation() {
    // Create a mock RWA claim
    let institutional_pubkey = [1u8; 32];
    let balance = 1_000_000_00; // $1M in cents
    let threshold = 500_000_00; // $500K threshold
    let signature = [2u8; 64];

    let claim = RwaClaim::new(
        institutional_pubkey,
        balance,
        threshold,
        signature,
    );

    assert_eq!(claim.balance, 1_000_000_00);
    assert_eq!(claim.threshold, 500_000_00);
    assert_eq!(claim.institutional_pubkey, institutional_pubkey);
}

#[test]
fn test_message_to_sign() {
    let claim = RwaClaim::new(
        [0u8; 32],
        12345,
        10000,
        [0u8; 64],
    );

    let message = claim.message_to_sign();
    assert_eq!(message, 12345u64.to_le_bytes());
}

#[test]
fn test_proving_mode_enum() {
    assert_eq!(ProvingMode::Mock, ProvingMode::Mock);
    assert_ne!(ProvingMode::Mock, ProvingMode::Stark);
    assert_ne!(ProvingMode::Stark, ProvingMode::Groth16);
}

#[test]
fn test_borsh_serialization() {
    let claim = RwaClaim::new(
        [1u8; 32],
        1_000_000,
        500_000,
        [2u8; 64],
    );

    // Serialize with Borsh
    let bytes = borsh::to_vec(&claim).unwrap();
    assert!(!bytes.is_empty());

    // Deserialize
    let deserialized = RwaClaim::try_from_slice(&bytes).unwrap();
    assert_eq!(claim.balance, deserialized.balance);
    assert_eq!(claim.threshold, deserialized.threshold);
}

#[test]
fn test_vkey_format_enum() {
    // Just verify the enum exists and can be used
    let _json = VkeyFormat::Json;
    let _binary = VkeyFormat::Binary;
    let _solidity = VkeyFormat::Solidity;
}

// Note: The following tests require a compiled SP1 guest program
// They are commented out but show the intended workflow

/*
#[test]
fn test_mock_proving() {
    use universal_privacy_engine_sp1::Sp1Backend;
    
    // Load guest ELF
    let elf = include_bytes!("../../guest/rwa_compliance/elf/riscv32im-succinct-zkvm-elf");
    let backend = Sp1Backend::new(elf.to_vec());

    // Create claim
    let claim = RwaClaim::new(
        [1u8; 32],
        1_000_000_00,
        500_000_00,
        [2u8; 64],
    );

    // Generate proof in mock mode (fast)
    let receipt = backend.prove_rwa(&claim, ProvingMode::Mock).unwrap();

    // Verify we got a receipt
    assert!(!receipt.proof.is_empty());
    assert!(!receipt.public_values.is_empty());
}

#[test]
fn test_verification_key_export() {
    use universal_privacy_engine_sp1::Sp1Backend;
    
    let elf = include_bytes!("../../guest/rwa_compliance/elf/riscv32im-succinct-zkvm-elf");
    let backend = Sp1Backend::new(elf.to_vec());

    // Export as JSON
    let vkey_json = backend.export_verification_key(VkeyFormat::Json).unwrap();
    assert!(!vkey_json.is_empty());

    // Export as binary
    let vkey_bin = backend.export_verification_key(VkeyFormat::Binary).unwrap();
    assert!(!vkey_bin.is_empty());
}
*/
