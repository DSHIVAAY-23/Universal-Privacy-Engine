//! Tool to capture (mock) zkTLS sessions and sign them as a Local Notary.
//!
//! Generates a `metadata.json` that matches the robust `RecordedTlsProof` structure.

use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "capture_zktls")]
struct Args {
    #[arg(long, default_value = "https://example.com/api/data")]
    url: String,

    #[arg(long, default_value = "fixtures/zktls")]
    out_dir: PathBuf,
}

#[derive(Serialize)]
struct RecordedTlsProof {
    domain: String,
    timestamp: u64,
    response_hash: String,
    cert_chain_hash: String,
    notary_pubkey: String,
    signature: String,
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // 1. Setup Data
    let mock_response = serde_json::json!({
        "data": {
            "balance": 5000,
            "currency": "USD",
            "last_updated_ts": 1735230000 
        }
    });
    let response_body = serde_json::to_string_pretty(&mock_response)?;
    
    let cert_chain_pem = "-----BEGIN CERTIFICATE-----\nMOCK_CERT_CHAIN_FOR_DEMO\n-----END CERTIFICATE-----";
    let timestamp = 1735128000; // Fixed timestamp for consistency
    let domain = "example.com";

    println!("Generating proof for {}", domain);

    // 2. Generate Notary Keypair
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let pubkey_hex = hex::encode(verifying_key.to_bytes());

    println!("Local Notary Public Key: {}", pubkey_hex);

    // 3. Compute Hashes
    let response_hash = sha256_hex(response_body.as_bytes());
    let cert_chain_hash = sha256_hex(cert_chain_pem.as_bytes());

    // 4. Construct Canonical Message
    // "domain:timestamp:response_hash:cert_chain_hash"
    let message = format!(
        "{}:{}:{}:{}",
        domain, timestamp, response_hash, cert_chain_hash
    );
    let message_bytes = message.as_bytes();

    // 5. Sign
    let signature = signing_key.sign(message_bytes);
    let sig_hex = hex::encode(signature.to_bytes());

    let proof = RecordedTlsProof {
        domain: domain.to_string(),
        timestamp,
        response_hash,
        cert_chain_hash,
        notary_pubkey: pubkey_hex,
        signature: sig_hex,
    };

    // 6. Write Files
    fs::create_dir_all(&args.out_dir)?;

    let meta_path = args.out_dir.join("metadata.json");
    let resp_path = args.out_dir.join("response_body.json");
    let cert_path = args.out_dir.join("cert_chain.pem");

    fs::write(&meta_path, serde_json::to_string_pretty(&proof)?)?;
    fs::write(&resp_path, &response_body)?;
    fs::write(&cert_path, cert_chain_pem)?;

    println!("✓ Fixtures written to {}", args.out_dir.display());
    println!("  - metadata.json");
    println!("  - response_body.json");
    println!("  - cert_chain.pem");

    Ok(())
}
