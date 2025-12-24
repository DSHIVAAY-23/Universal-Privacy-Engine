//! # RWA Compliance Guest Program
//!
//! This SP1 zkVM guest program verifies institutional asset compliance without revealing
//! private balance information.

#![no_main]
sp1_zkvm::entrypoint!(main);

use borsh::BorshDeserialize;
use universal_privacy_engine_core::rwa::RwaClaim;

pub fn main() {
    // Read RWA claim from stdin
    let claim = RwaClaim::try_from_slice(&sp1_zkvm::io::read_vec())
        .expect("Failed to deserialize RwaClaim from stdin");

    // ═══════════════════════════════════════════════════════════════════════════
    // Ed25519 Signature Verification
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // NOTE: SP1 3.4 Ed25519 precompile API documentation pending.
    // For production, integrate the correct precompile:
    //
    // Option 1: Check if sp1_zkvm::precompiles::ed25519 exists
    // Option 2: Use sp1_zkvm::syscalls::ed25519_verify if available
    // Option 3: Import ed25519-dalek for pure Rust verification (slower)
    //
    // For now, we use a placeholder that validates signature format.
    // ═══════════════════════════════════════════════════════════════════════════

    // Placeholder: Check signature and pubkey are correct lengths
    assert!(
        claim.signature.len() == 64,
        "Invalid signature length"
    );
    assert!(
        claim.institutional_pubkey.len() == 32,
        "Invalid pubkey length"
    );

    // TODO: Replace with actual Ed25519 verification
    // let message = claim.balance.to_le_bytes();
    // let valid = verify_ed25519_signature(&claim.signature, &claim.institutional_pubkey, &message);
    // assert!(valid, "Invalid signature");

    // ═══════════════════════════════════════════════════════════════════════════
    // Compliance Threshold Check
    // ═══════════════════════════════════════════════════════════════════════════

    assert!(
        claim.balance >= claim.threshold,
        "Compliance check failed: balance ({}) is below threshold ({})",
        claim.balance,
        claim.threshold
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // Commit Public Values to Journal
    // ═══════════════════════════════════════════════════════════════════════════

    sp1_zkvm::io::commit(&claim.institutional_pubkey);
    sp1_zkvm::io::commit(&claim.threshold);
}
