//! # VeriVault MCP Server
//!
//! MCP server implementation for Cursor/Claude integration.
//! Provides stdio-based communication for tool execution.

use crate::mcp::tools::{get_tool_definitions, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use universal_privacy_engine_core::{
    agent::{DataSource, StructuredExtractor},
    logging::ZkAuditTrail,
    rwa::RwaClaim,
};

/// MCP Server for VeriVault Agent
pub struct VeriVaultMcpServer {
    extractor: StructuredExtractor,
    audit_trail: ZkAuditTrail,
}

impl VeriVaultMcpServer {
    /// Create a new MCP server
    pub fn new() -> Self {
        Self {
            extractor: StructuredExtractor::new(),
            audit_trail: ZkAuditTrail::new(),
        }
    }

    /// Run the MCP server (stdio mode)
    pub fn run(&mut self) -> io::Result<()> {
        eprintln!("🚀 VeriVault MCP Server starting...");
        eprintln!("📡 Listening on stdio for MCP requests");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            
            // Parse MCP request
            match serde_json::from_str::<McpRequest>(&line) {
                Ok(request) => {
                    let response = self.handle_request(request);
                    let response_json = serde_json::to_string(&response)?;
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse request: {}", e);
                    let error_response = McpResponse {
                        id: None,
                        result: None,
                        error: Some(format!("Invalid request: {}", e)),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }

    /// Handle an MCP request
    fn handle_request(&mut self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "tools/list" => self.handle_list_tools(request.id),
            "tools/call" => self.handle_tool_call(request.id, request.params),
            _ => McpResponse {
                id: request.id,
                result: None,
                error: Some(format!("Unknown method: {}", request.method)),
            },
        }
    }

    /// Handle list tools request
    fn handle_list_tools(&self, id: Option<String>) -> McpResponse {
        let tools = get_tool_definitions();
        McpResponse {
            id,
            result: Some(json!({ "tools": tools })),
            error: None,
        }
    }

    /// Handle tool call request
    fn handle_tool_call(&mut self, id: Option<String>, params: Option<Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return McpResponse {
                    id,
                    result: None,
                    error: Some("Missing parameters".to_string()),
                }
            }
        };

        let tool_name = params["name"].as_str().unwrap_or("");
        let arguments = &params["arguments"];

        let result = match tool_name {
            "extract_claim" => self.execute_extract_claim(arguments),
            "generate_compliance_proof" => self.execute_generate_proof(arguments),
            "submit_to_chain" => self.execute_submit_to_chain(arguments),
            "list_verifiers" => self.execute_list_verifiers(),
            _ => ToolResult::error(format!("Unknown tool: {}", tool_name)),
        };

        McpResponse {
            id,
            result: Some(json!(result)),
            error: None,
        }
    }

    /// Execute extract_claim tool
    fn execute_extract_claim(&mut self, args: &Value) -> ToolResult {
        let raw_data = match args["raw_data"].as_str() {
            Some(data) => data,
            None => return ToolResult::error("Missing raw_data parameter".to_string()),
        };

        let threshold = match args["threshold"].as_u64() {
            Some(t) => t,
            None => return ToolResult::error("Missing threshold parameter".to_string()),
        };

        // Create data source
        let source = DataSource::Text(raw_data.to_string());

        // Extract claim
        match self.extractor.extract(source) {
            Ok(mut result) => {
                // Override threshold with user-provided value
                result.claim.threshold = threshold;

                // Log to audit trail
                self.audit_trail.add_entry(
                    universal_privacy_engine_core::logging::AgentAction::ExtractClaim,
                    raw_data.as_bytes(),
                    &serde_json::to_vec(&result.claim).unwrap(),
                    b"extract_claim_v1",
                    result.confidence,
                );

                ToolResult::success(json!({
                    "claim": {
                        "institutional_pubkey": result.claim.institutional_pubkey.to_vec(),
                        "balance": result.claim.balance,
                        "threshold": result.claim.threshold,
                        "signature": result.claim.signature.to_vec(),
                    },
                    "confidence": result.confidence,
                    "warnings": result.warnings,
                    "metadata": result.metadata,
                }))
            }
            Err(e) => ToolResult::error(format!("Extraction failed: {}", e)),
        }
    }

    /// Execute generate_compliance_proof tool
    fn execute_generate_proof(&mut self, args: &Value) -> ToolResult {
        // Placeholder - would integrate with SP1 backend
        ToolResult::success(json!({
            "proof_receipt": {
                "proof": "placeholder_proof_bytes",
                "public_values": "placeholder_public_values",
                "metadata": "placeholder_metadata"
            },
            "proof_hash": "0x1a2b3c4d5e6f...",
            "generation_time_ms": 2300,
            "mode": "groth16"
        }))
    }

    /// Execute submit_to_chain tool
    fn execute_submit_to_chain(&mut self, args: &Value) -> ToolResult {
        let chain = args["chain"].as_str().unwrap_or("unknown");

        // Placeholder - would integrate with chain orchestrator
        ToolResult::success(json!({
            "transaction_hash": format!("{}_tx_placeholder", chain),
            "verification_status": true,
            "explorer_url": format!("https://explorer.{}.com/tx/placeholder", chain),
            "gas_used": 250000
        }))
    }

    /// Execute list_verifiers tool
    fn execute_list_verifiers(&self) -> ToolResult {
        ToolResult::success(json!({
            "verifiers": [
                {
                    "chain": "solana",
                    "contract_address": "solana_verifier_placeholder",
                    "status": "active",
                    "verification_count": 42
                },
                {
                    "chain": "stellar",
                    "contract_address": "stellar_verifier_placeholder",
                    "status": "active",
                    "verification_count": 28
                },
                {
                    "chain": "mantra-cosmwasm",
                    "contract_address": "mantra_verifier_placeholder",
                    "status": "active",
                    "verification_count": 15
                }
            ]
        }))
    }

    /// Export audit trail
    pub fn export_audit_trail(&self) -> String {
        self.audit_trail.export_json().unwrap_or_else(|_| "{}".to_string())
    }
}

impl Default for VeriVaultMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

/// MCP Request format
#[derive(Debug, Deserialize)]
struct McpRequest {
    id: Option<String>,
    method: String,
    params: Option<Value>,
}

/// MCP Response format
#[derive(Debug, Serialize)]
struct McpResponse {
    id: Option<String>,
    result: Option<Value>,
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = VeriVaultMcpServer::new();
        assert_eq!(server.audit_trail.len(), 0);
    }

    #[test]
    fn test_list_tools() {
        let server = VeriVaultMcpServer::new();
        let response = server.handle_list_tools(Some("test-id".to_string()));
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_extract_claim() {
        let mut server = VeriVaultMcpServer::new();
        let args = json!({
            "raw_data": "Chase Bank\nAccount Balance: $50,000.00",
            "threshold": 5000000
        });

        let result = server.execute_extract_claim(&args);
        assert!(result.success);
    }
}
