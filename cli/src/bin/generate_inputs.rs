//! # RWA Credentials Generator - "Institutional Fuel"
//!
//! This binary simulates an institutional bank generating cryptographically valid
//! RWA compliance credentials for testing the Universal Privacy Engine.
//!
//! ## What It Does
//! 1. Generates an Ed25519 institutional keypair
//! 2. Creates a Merkle tree of user balances (simulating a bank ledger)
//! 3. Selects a test user and generates their Merkle inclusion proof
//! 4. Signs the user's balance with the institutional private key
//! 5. Serializes everything to binary format (Borsh) for the zkVM

use clap::Parser;
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use rs_merkle::{MerkleTree, algorithms::Sha256};
use sha2::{Sha256 as Sha256Hasher, Digest};
use borsh::{BorshSerialize, to_vec};
use std::fs::File;
use std::io::Write;

/// RWA Claim with Merkle proof (matches guest program struct)
#[derive(BorshSerialize)]
pub struct RwaClaimWithProof {
    pub institutional_pubkey: [u8; 32],
    pub balance: u64,
    pub threshold: u64,
    pub signature: [u8; 64],
    pub merkle_root: [u8; 32],
    pub merkle_proof: Vec<[u8; 32]>,
    pub leaf_index: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate cryptographically valid RWA credentials for testing", long_about = None)]
struct Args {
    /// Output file path for the generated credentials
    #[arg(short, long, default_value = "rwa_creds.bin")]
    output: String,
    
    /// User balance in cents (default: $1.5M)
    #[arg(short, long, default_value = "150000000")]
    balance: u64,
    
    /// Compliance threshold in cents (default: $1.0M)
    #[arg(short, long, default_value = "100000000")]
    threshold: u64,
}

fn main() {
    let args = Args::parse();
    
    println!("🏦 Universal Privacy Engine - Institutional Credentials Generator");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Step 1: Setup the "Bank" (Institutional Key)
    println!("\n🔑 Step 1: Generating institutional Ed25519 keypair...");
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let pubkey_bytes = verifying_key.to_bytes();
    
    println!("   ✅ Institutional Public Key: {}", hex::encode(pubkey_bytes));
    
    // Step 2: Setup the "User" (Balance & Threshold)
    println!("\n💰 Step 2: Setting up user credentials...");
    let user_balance = args.balance;
    let threshold = args.threshold;
    
    println!("   Balance: ${}.00", user_balance / 100);
    println!("   Threshold: ${}.00", threshold / 100);
    println!("   Compliance: {}", if user_balance >= threshold { "✅ PASS" } else { "❌ FAIL" });
    
    // Step 3: Bank signs the User's Balance
    println!("\n✍️  Step 3: Signing user balance with institutional key...");
    let balance_bytes = user_balance.to_le_bytes();
    let signature = signing_key.sign(&balance_bytes);
    
    println!("   ✅ Signature: {}", hex::encode(signature.to_bytes()));
    
    // Step 4: Create a Merkle Tree (The Ledger)
    println!("\n🌳 Step 4: Building institutional Merkle tree (ledger)...");
    
    // Create 10 fake accounts to simulate a real bank ledger
    let leaf_values = vec![
        5_000_000,    // $50k
        10_000_000,   // $100k
        7_500_000,    // $75k
        user_balance, // Our test user
        2_000_000,    // $20k
        15_000_000,   // $150k
        3_000_000,    // $30k
        8_000_000,    // $80k
        1_000_000,    // $10k
        12_000_000,   // $120k
    ];
    
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|balance| {
            let mut hasher = Sha256Hasher::new();
            hasher.update(balance.to_le_bytes());
            hasher.finalize().into()
        })
        .collect();
    
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let root = merkle_tree.root().expect("Tree must have root");
    
    println!("   Total accounts: {}", leaf_values.len());
    println!("   ✅ Merkle Root: {}", hex::encode(root));
    
    // Step 5: Get proof for our user (index 3)
    println!("\n🔐 Step 5: Generating Merkle inclusion proof...");
    let user_index = 3; // Our test user is at index 3
    let proof = merkle_tree.proof(&[user_index]);
    let proof_hashes: Vec<[u8; 32]> = proof.proof_hashes()
        .iter()
        .map(|h| {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(h);
            arr
        })
        .collect();
    
    println!("   User index: {}", user_index);
    println!("   ✅ Proof length: {} hashes", proof_hashes.len());
    
    // Step 6: Serialize Everything
    println!("\n📦 Step 6: Serializing credentials to binary (Borsh)...");
    let claim = RwaClaimWithProof {
        institutional_pubkey: pubkey_bytes,
        balance: user_balance,
        threshold,
        signature: signature.to_bytes(),
        merkle_root: root,
        merkle_proof: proof_hashes,
        leaf_index: user_index,
    };
    
    let encoded = to_vec(&claim).expect("Failed to serialize claim");
    
    let mut file = File::create(&args.output).expect("Failed to create output file");
    file.write_all(&encoded).expect("Failed to write to file");
    
    println!("   ✅ Serialized {} bytes", encoded.len());
    println!("   ✅ Saved to: {}", args.output);
    
    // Summary
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Institutional credentials generated successfully!");
    println!("\n📊 Summary:");
    println!("   • File: {}", args.output);
    println!("   • Size: {} bytes", encoded.len());
    println!("   • Institutional pubkey: {}", hex::encode(pubkey_bytes));
    println!("   • Merkle root: {}", hex::encode(root));
    println!("   • User balance: ${}.00", user_balance / 100);
    println!("   • Threshold: ${}.00", threshold / 100);
    println!("   • Compliance: {}", if user_balance >= threshold { "✅ PASS" } else { "❌ FAIL" });
    
    println!("\n🚀 Next steps:");
    println!("   1. Run: cargo run --bin upe -- prove --input {}", args.output);
    println!("   2. Watch the ZK proof generation!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
