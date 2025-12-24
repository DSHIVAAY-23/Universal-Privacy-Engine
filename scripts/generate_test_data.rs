//! # Institutional Test Data Generator
//!
//! This script simulates an institutional bank generating RWA compliance data:
//! 1. Creates a Merkle tree of user balances
//! 2. Signs specific user data with Ed25519
//! 3. Exports test data for ZK proof generation

use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use rs_merkle::{MerkleTree, algorithms::Sha256};
use serde::{Serialize, Deserialize};
use sha2::{Sha256 as Sha256Hasher, Digest};
use std::fs;

/// RWA Claim with Merkle proof (matches guest program struct)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RwaClaimWithProof {
    pub institutional_pubkey: [u8; 32],
    pub balance: u64,
    pub threshold: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub signature: [u8; 64],
    pub merkle_root: [u8; 32],
    pub merkle_proof: Vec<[u8; 32]>,
    pub leaf_index: usize,
}

/// User account with balance
#[derive(Debug)]
struct UserAccount {
    id: u32,
    balance: u64, // in cents
}

fn main() {
    println!("🏦 Universal Privacy Engine - Institutional Test Data Generator");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Step 1: Generate institutional Ed25519 keypair
    println!("\n📝 Step 1: Generating institutional Ed25519 keypair...");
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    println!("   ✅ Institutional Public Key: {}", hex::encode(verifying_key.to_bytes()));
    
    // Step 2: Create dummy user accounts (simulating a bank's customer database)
    println!("\n💰 Step 2: Creating 10 dummy user accounts...");
    let users = vec![
        UserAccount { id: 1, balance: 5_000_000 },   // $50,000
        UserAccount { id: 2, balance: 10_000_000 },  // $100,000
        UserAccount { id: 3, balance: 7_500_000 },   // $75,000
        UserAccount { id: 4, balance: 2_000_000 },   // $20,000
        UserAccount { id: 5, balance: 15_000_000 },  // $150,000 (our test user)
        UserAccount { id: 6, balance: 3_000_000 },   // $30,000
        UserAccount { id: 7, balance: 8_000_000 },   // $80,000
        UserAccount { id: 8, balance: 1_000_000 },   // $10,000
        UserAccount { id: 9, balance: 12_000_000 },  // $120,000
        UserAccount { id: 10, balance: 6_000_000 },  // $60,000
    ];
    
    for user in &users {
        println!("   User {}: ${}.00", user.id, user.balance / 100);
    }
    
    // Step 3: Build Merkle tree from user balances
    println!("\n🌳 Step 3: Building Merkle tree from user balances...");
    let leaves: Vec<[u8; 32]> = users.iter().map(|user| {
        let mut hasher = Sha256Hasher::new();
        hasher.update(&user.balance.to_le_bytes());
        hasher.finalize().into()
    }).collect();
    
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let merkle_root = merkle_tree.root().expect("Failed to get Merkle root");
    
    println!("   ✅ Merkle Root: {}", hex::encode(merkle_root));
    
    // Step 4: Select test user (User 5 with $150k balance)
    let test_user_index = 4; // User 5 (0-indexed)
    let test_user = &users[test_user_index];
    
    println!("\n👤 Step 4: Selecting test user for proof...");
    println!("   User ID: {}", test_user.id);
    println!("   Balance: ${}.00", test_user.balance / 100);
    
    // Step 5: Generate Merkle inclusion proof for test user
    println!("\n🔐 Step 5: Generating Merkle inclusion proof...");
    let proof = merkle_tree.proof(&[test_user_index]);
    let proof_hashes: Vec<[u8; 32]> = proof.proof_hashes()
        .iter()
        .map(|h| {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(h);
            arr
        })
        .collect();
    
    println!("   ✅ Proof length: {} hashes", proof_hashes.len());
    
    // Step 6: Sign the user's balance with institutional private key
    println!("\n✍️  Step 6: Signing user balance with institutional key...");
    let message = test_user.balance.to_le_bytes();
    let signature = signing_key.sign(&message);
    
    println!("   ✅ Signature: {}", hex::encode(signature.to_bytes()));
    
    // Step 7: Create RwaClaimWithProof
    let threshold = 10_000_000; // $100k threshold
    
    println!("\n📋 Step 7: Creating RWA Claim...");
    println!("   Balance: ${}.00", test_user.balance / 100);
    println!("   Threshold: ${}.00", threshold / 100);
    println!("   Compliance: {}", if test_user.balance >= threshold { "✅ PASS" } else { "❌ FAIL" });
    
    let claim = RwaClaimWithProof {
        institutional_pubkey: verifying_key.to_bytes(),
        balance: test_user.balance,
        threshold,
        signature: signature.to_bytes(),
        merkle_root,
        merkle_proof: proof_hashes.clone(),
        leaf_index: test_user_index,
    };
    
    // Step 8: Export to JSON
    println!("\n💾 Step 8: Exporting test data to JSON...");
    let json = serde_json::to_string_pretty(&claim).expect("Failed to serialize claim");
    fs::write("test_input.json", &json).expect("Failed to write test_input.json");
    
    println!("   ✅ Saved to: test_input.json");
    println!("   File size: {} bytes", json.len());
    
    // Summary
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Test data generation complete!");
    println!("\n📊 Summary:");
    println!("   • Total users in tree: {}", users.len());
    println!("   • Test user balance: ${}.00", test_user.balance / 100);
    println!("   • Compliance threshold: ${}.00", threshold / 100);
    println!("   • Merkle proof length: {} hashes", proof_hashes.len());
    println!("   • Institutional pubkey: {}", hex::encode(verifying_key.to_bytes()));
    println!("\n🚀 Next steps:");
    println!("   1. Run: cargo run --bin upe -- demo-compliance");
    println!("   2. Watch the ZK proof generation!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
