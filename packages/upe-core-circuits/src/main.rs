use axum::{
    routing::post,
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct ProveRequest {
    #[serde(rename = "assetContract")]
    asset_contract: String,
    #[serde(rename = "minRequiredValue")]
    min_required_value: String,
}

#[derive(Serialize)]
struct ProveResponse {
    source: String,
    proof: ProofData,
    #[serde(rename = "publicSignals")]
    public_signals: Vec<String>,
    #[serde(rename = "nullifierHash")]
    nullifier_hash: String,
    #[serde(rename = "assetContract")]
    asset_contract: String,
    #[serde(rename = "minRequiredValue")]
    min_required_value: String,
    timestamp: u64,
}

#[derive(Serialize)]
struct ProofData {
    pi_a: [String; 3],
    pi_b: [[String; 2]; 3],
    pi_c: [String; 3],
    protocol: String,
    curve: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/prove", post(generate_proof));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("UPE Rust Prover Node listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_proof(Json(payload): Json<ProveRequest>) -> Json<ProveResponse> {
    println!("[PROVER] Received request for contract: {} with min value: {}", payload.asset_contract, payload.min_required_value);
    
    // Simulate complex proof generation time
    tokio::time::sleep(tokio::time::Duration::from_millis(900)).await;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Deterministic mock generation based on timestamp to simulate uniqueness
    let dummy_hex = || format!("0x{:064x}", rand::random::<u128>());

    let nullifier = format!("0x{:064x}", rand::random::<u128>());

    let proof = ProofData {
        pi_a: [dummy_hex(), dummy_hex(), "0x01".to_string()],
        pi_b: [
            [dummy_hex(), dummy_hex()],
            [dummy_hex(), dummy_hex()],
            ["0x01".to_string(), "0x00".to_string()]
        ],
        pi_c: [dummy_hex(), dummy_hex(), "0x01".to_string()],
        protocol: "groth16".to_string(),
        curve: "bn128".to_string(),
    };

    let response = ProveResponse {
        source: "rust-backend".to_string(),
        proof,
        public_signals: vec![
            dummy_hex(),
            payload.min_required_value.clone(),
            nullifier.clone(),
        ],
        nullifier_hash: nullifier,
        asset_contract: payload.asset_contract,
        min_required_value: payload.min_required_value,
        timestamp,
    };

    Json(response)
}
