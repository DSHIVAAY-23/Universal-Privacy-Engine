//! # RWA Compliance Guest Program (Production-Grade)
//!
//! This SP1 zkVM guest program verifies institutional asset compliance using:
//! 1. Ed25519 signature verification (proves data authenticity)
//! 2. Merkle inclusion proof (proves balance is part of institutional tree)

#![no_main]
sp1_zkvm::entrypoint!(main);

use borsh::{BorshDeserialize, BorshSerialize};
use rs_merkle::{MerkleProof, algorithms::Sha256};
use sha2::{Sha256 as Sha256Hasher, Digest};

/// RWA Claim with Merkle proof
#[derive(BorshSerialize, BorshDeserialize)]
pub struct RwaClaimWithProof {
    /// Institutional public key (32 bytes)
    pub institutional_pubkey: [u8; 32],
    
    /// User's balance (private, in cents)
    pub balance: u64,
    
    /// Compliance threshold (public, in cents)
    pub threshold: u64,
    
    /// Ed25519 signature over balance (64 bytes)
    pub signature: [u8; 64],
    
    /// Merkle root of all balances (public)
    pub merkle_root: [u8; 32],
    
    /// Merkle proof path (private)
    pub merkle_proof: Vec<[u8; 32]>,
    
    /// Leaf index in Merkle tree
    pub leaf_index: usize,
}

pub fn main() {
    // Read and deserialize input
    let input_bytes = sp1_zkvm::io::read_vec();
    let claim = RwaClaimWithProof::try_from_slice(&input_bytes)
        .expect("Failed to deserialize RwaClaimWithProof");

    // ═══════════════════════════════════════════════════════════════════════════
    // Step 1: Ed25519 Signature Verification (REAL SP1 PRECOMPILE)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This uses the SP1 hardware accelerator for Ed25519 verification.
    // This is 10-100x faster than pure Rust implementation.
    //
    // CRITICAL: We use ed25519-dalek with SP1's optimized syscalls
    // ═══════════════════════════════════════════════════════════════════════════
    
    use ed25519_dalek::{Verifier, VerifyingKey, Signature};
    
    let message = claim.balance.to_le_bytes();
    
    // Create Ed25519 verifying key from institutional pubkey
    let verifying_key = VerifyingKey::from_bytes(&claim.institutional_pubkey)
        .expect("Invalid Ed25519 public key");
    
    // Create signature from bytes
    let signature = Signature::from_bytes(&claim.signature);
    
    // VERIFY SIGNATURE - This will PANIC if invalid!
    // SP1 optimizes the underlying curve operations via syscalls
    verifying_key.verify(&message, &signature)
        .expect("Ed25519 signature verification failed");

    // ═══════════════════════════════════════════════════════════════════════════
    // Step 2: Merkle Inclusion Proof Verification
    // ═══════════════════════════════════════════════════════════════════════════
    
    // Compute leaf hash
    let mut hasher = Sha256Hasher::new();
    hasher.update(&claim.balance.to_le_bytes());
    let leaf_hash: [u8; 32] = hasher.finalize().into();
    
    // Verify Merkle proof
    let merkle_valid = verify_merkle_proof(
        &claim.merkle_root,
        &claim.merkle_proof,
        &leaf_hash,
        claim.leaf_index,
    );
    
    assert!(
        merkle_valid,
        "Merkle proof verification failed"
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // Step 3: Compliance Threshold Check
    // ═══════════════════════════════════════════════════════════════════════════
    
    assert!(
        claim.balance >= claim.threshold,
        "Compliance check failed: balance {} < threshold {}",
        claim.balance,
        claim.threshold
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // Step 4: Commit Public Values
    // ═══════════════════════════════════════════════════════════════════════════
    
    sp1_zkvm::io::commit(&claim.institutional_pubkey);
    sp1_zkvm::io::commit(&claim.threshold);
    sp1_zkvm::io::commit(&claim.merkle_root);
}

/// Verify Merkle inclusion proof
fn verify_merkle_proof(
    root: &[u8; 32],
    proof: &[[u8; 32]],
    leaf: &[u8; 32],
    index: usize,
) -> bool {
    let proof_hashes: Vec<[u8; 32]> = proof.to_vec();
    let indices = vec![index];
    
    let merkle_proof = MerkleProof::<Sha256>::new(proof_hashes);
    
    let leaves_to_prove = vec![*leaf];
    merkle_proof.verify(
        *root,
        &indices,
        &leaves_to_prove,
        proof.len() + 1,
    )
}
