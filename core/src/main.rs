//! Universal Privacy Engine - Notary REST API Server
//!
//! This server exposes the Notary signing functionality via HTTP endpoints
//! for the frontend to consume. It generates STLOP (Signed TLS-Originated Proofs)
//! that can be verified on-chain by Ethereum smart contracts.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use universal_privacy_engine_core::notary::{
    GenerateProofRequest, HealthResponse, NotarySigner, STLOPProof,
};

/// Shared application state
#[derive(Clone)]
struct AppState {
    notary: Arc<NotarySigner>,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load notary private key from environment
    let notary_private_key = std::env::var("NOTARY_PRIVATE_KEY")
        .expect("NOTARY_PRIVATE_KEY must be set in .env file");

    // Create notary signer
    let notary = NotarySigner::new(&notary_private_key)
        .expect("Failed to create notary signer");

    let notary_address = format!("0x{:x}", notary.address());
    println!("🔐 Notary Service Starting...");
    println!("📍 Notary Address: {}", notary_address);

    // Create shared state
    let state = AppState {
        notary: Arc::new(notary),
    };

    // Configure CORS to allow all origins and headers.
    // allow_headers(Any) is required so the browser CORS preflight passes
    // the `ngrok-skip-browser-warning` header sent by the Vercel frontend.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/healthz", get(health_check))          // liveness probe
        .route("/api/generate-proof", post(generate_proof))
        .layer(cors)
        .with_state(state);

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Server listening on http://{}", addr);
    println!("📡 Endpoints:");
    println!("   GET  /api/health");
    println!("   GET  /healthz         (liveness probe)");
    println!("   POST /api/generate-proof");
    println!("");

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

/// Health check endpoint
///
/// GET /api/health
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        notary_address: format!("0x{:x}", state.notary.address()),
    })
}

/// Generate STLOP proof endpoint
///
/// POST /api/generate-proof
/// Body: { "employee_address": "0x..." }
async fn generate_proof(
    State(state): State<AppState>,
    Json(request): Json<GenerateProofRequest>,
) -> Result<Json<STLOPProof>, (StatusCode, Json<serde_json::Value>)> {
    // Validate employee address format
    if !request.employee_address.starts_with("0x") || request.employee_address.len() != 42 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid employee address format. Expected 0x-prefixed 40-character hex string"
            })),
        ));
    }

    // Generate proof
    match state.notary.generate_proof(&request.employee_address).await {
        Ok(proof) => {
            println!("✅ Generated proof for employee: {}", request.employee_address);
            Ok(Json(proof))
        }
        Err(e) => {
            eprintln!("❌ Failed to generate proof: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Failed to generate proof: {}", e)
                })),
            ))
        }
    }
}
