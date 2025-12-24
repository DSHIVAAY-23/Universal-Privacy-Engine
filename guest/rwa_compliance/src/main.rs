//! # RWA Compliance Guest Program
//!
//! This SP1 zkVM guest program verifies institutional asset compliance without revealing
//! private balance information. It demonstrates the power of zero-knowledge proofs for
//! regulatory compliance in decentralized finance.
//!
//! ## What This Program Proves
//!
//! Given a private RWA claim containing:
//! - An institution's Ed25519 public key
//! - A private balance amount
//! - A compliance threshold
//! - An Ed25519 signature over the balance
//!
//! This program proves:
//! 1. The signature is valid (institution signed the balance)
//! 2. The balance meets or exceeds the threshold
//! 3. WITHOUT revealing the actual balance amount
//!
//! ## ZK-VM Architecture
//!
//! This code runs inside the SP1 RISC-V zkVM, not on the host machine.
//! Every instruction executed here becomes part of the zero-knowledge proof.
//!
//! ### Performance Optimization: Precompiles
//!
//! We use `sp1_zkvm::lib::verify_ed25519` instead of a pure Rust Ed25519 library.
//! This syscall invokes an optimized ZK circuit for Ed25519 verification, providing:
//! - 10-100x faster proof generation
//! - Smaller proof size
//! - Lower on-chain verification gas costs
//!
//! This is the "Deep Tech" advantage - understanding when to use precompiles vs
//! generic computation is critical for production ZK systems.

#![no_main]
sp1_zkvm::entrypoint!(main);

use borsh::BorshDeserialize;
use universal_privacy_engine_core::rwa::RwaClaim;

/// Main entry point for the RWA Compliance zkVM guest program.
///
/// # Execution Flow
///
/// 1. **Read Private Input**: Deserialize RwaClaim from stdin (private to the prover)
/// 2. **Verify Signature**: Use Ed25519 precompile to verify institutional signature
/// 3. **Check Compliance**: Assert that balance >= threshold
/// 4. **Commit Public Values**: Write institutional pubkey and threshold to journal
///
/// # Privacy Guarantees
///
/// - The actual balance is NEVER written to the public journal
/// - Only the threshold and institutional identity are revealed
/// - The proof cryptographically guarantees balance >= threshold without disclosing balance
///
/// # Panics
///
/// This program will panic (and proof generation will fail) if:
/// - The Ed25519 signature is invalid
/// - The balance is below the threshold
/// - Input deserialization fails
pub fn main() {
    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 1: Read Private Input from Host
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // The host (prover) feeds the RwaClaim through stdin. This data is PRIVATE -
    // it's not included in the proof or visible to verifiers.
    //
    // Using Borsh for deserialization because:
    // - Deterministic encoding (same struct -> same bytes)
    // - Minimal overhead in constrained zkVM environment
    // - Blockchain-native (used by Solana, NEAR)

    let claim = RwaClaim::try_from_slice(&sp1_zkvm::io::read_vec())
        .expect("Failed to deserialize RwaClaim from stdin");

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 2: Verify Ed25519 Signature (Using SP1 Precompile)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // CRITICAL OPTIMIZATION: We use sp1_zkvm::lib::verify_ed25519 instead of
    // a pure Rust Ed25519 library (like ed25519-dalek).
    //
    // Why? SP1 has an optimized ZK circuit for Ed25519 verification. When we call
    // this precompile, the prover:
    // 1. Executes native Ed25519 verification (fast)
    // 2. Generates a witness for the optimized subcircuit
    // 3. Includes the subcircuit proof in the final proof
    //
    // Result: ~10-100x faster than proving generic RISC-V Ed25519 code.
    //
    // The message being signed is the balance (as 8 bytes, little-endian).
    // In production, you'd include a timestamp and nonce to prevent replay attacks.

    let message = claim.balance.to_le_bytes();

    let signature_valid = sp1_zkvm::lib::verify_ed25519(
        &claim.signature,
        &claim.institutional_pubkey,
        &message,
    );

    // If signature is invalid, panic. This will cause proof generation to fail.
    // The verifier will never see an invalid proof - it simply won't be generated.
    assert!(
        signature_valid,
        "Ed25519 signature verification failed - institution did not sign this balance"
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 3: Verify Compliance Threshold
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This is the core compliance check: does the institution hold enough assets?
    //
    // The balance is PRIVATE - it's never revealed. But the proof will
    // cryptographically guarantee this assertion passed.

    assert!(
        claim.balance >= claim.threshold,
        "Compliance check failed: balance ({}) is below threshold ({})",
        claim.balance,
        claim.threshold
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 4: Commit Public Values to Journal
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // The journal contains the PUBLIC outputs of this computation.
    // These values will be included in the proof and visible to verifiers.
    //
    // We commit:
    // - Institutional public key (identity)
    // - Threshold (compliance requirement)
    //
    // We DO NOT commit:
    // - Balance (remains private)
    // - Signature (verification happened privately)
    //
    // This is the essence of zero-knowledge: proving a property (balance >= threshold)
    // without revealing the underlying data (actual balance).

    sp1_zkvm::io::commit(&claim.institutional_pubkey);
    sp1_zkvm::io::commit(&claim.threshold);

    // ═══════════════════════════════════════════════════════════════════════════
    // Proof Generation Complete
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // When this program exits successfully:
    // 1. The SP1 prover generates a cryptographic proof of execution
    // 2. The proof includes the public journal (pubkey + threshold)
    // 3. Anyone can verify the proof without re-running this code
    // 4. The verifier learns ONLY what's in the journal - nothing else
    //
    // This proof can be:
    // - Verified off-chain (fast, ~1ms)
    // - Wrapped in Groth16 for on-chain verification (Solana, Stellar, EVM)
    // - Aggregated with other proofs for batch verification
}

// ═══════════════════════════════════════════════════════════════════════════════
// Future Enhancements
// ═══════════════════════════════════════════════════════════════════════════════
//
// 1. **Timestamp Validation**: Add a timestamp to RwaClaim and verify it's recent
//    to prevent replay attacks.
//
// 2. **Multi-Asset Support**: Extend RwaClaim to include multiple asset types
//    (e.g., BTC, ETH, stablecoins) with per-asset thresholds.
//
// 3. **Range Proofs**: Instead of just >= threshold, prove balance is in a range
//    (e.g., between $1M and $10M) for more granular compliance.
//
// 4. **Merkle Proofs**: Verify the institution is part of an approved whitelist
//    using a Merkle tree commitment.
//
// 5. **SHA256 Precompile**: If we add hashing (e.g., for Merkle proofs), use
//    sp1_zkvm::lib::sha256 for another 10-100x speedup.
