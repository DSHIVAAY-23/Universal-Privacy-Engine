//! # MCP Tool Definitions
//!
//! Defines the tools exposed to Cursor/Claude via MCP.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool definition for MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub data: Value,
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(data: Value) -> Self {
        Self {
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: Value::Null,
            error: Some(message),
        }
    }
}

/// Get all tool definitions
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        extract_claim_tool(),
        generate_proof_tool(),
        submit_to_chain_tool(),
        list_verifiers_tool(),
    ]
}

/// Extract claim tool definition
fn extract_claim_tool() -> ToolDefinition {
    ToolDefinition {
        name: "extract_claim".to_string(),
        description: "Extract RWA compliance claim from raw data (bank statement, financial document)".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "raw_data": {
                    "type": "string",
                    "description": "Raw text from bank statement or financial document"
                },
                "data_format": {
                    "type": "string",
                    "enum": ["text", "json", "csv"],
                    "description": "Format of the input data",
                    "default": "text"
                },
                "threshold": {
                    "type": "number",
                    "description": "Minimum balance required for compliance (in cents)"
                }
            },
            "required": ["raw_data", "threshold"]
        }),
    }
}

/// Generate proof tool definition
fn generate_proof_tool() -> ToolDefinition {
    ToolDefinition {
        name: "generate_compliance_proof".to_string(),
        description: "Generate a Groth16 ZK proof for RWA compliance".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "claim": {
                    "type": "object",
                    "description": "RwaClaim object from extract_claim",
                    "properties": {
                        "institutional_pubkey": {"type": "array"},
                        "balance": {"type": "number"},
                        "threshold": {"type": "number"},
                        "signature": {"type": "array"}
                    }
                },
                "mode": {
                    "type": "string",
                    "enum": ["mock", "stark", "groth16"],
                    "default": "groth16",
                    "description": "Proving mode (mock for testing, groth16 for production)"
                }
            },
            "required": ["claim"]
        }),
    }
}

/// Submit to chain tool definition
fn submit_to_chain_tool() -> ToolDefinition {
    ToolDefinition {
        name: "submit_to_chain".to_string(),
        description: "Submit proof to on-chain verifier".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "proof_receipt": {
                    "type": "object",
                    "description": "ProofReceipt from generate_compliance_proof"
                },
                "chain": {
                    "type": "string",
                    "enum": ["solana", "stellar", "mantra-evm", "mantra-cosmwasm"],
                    "description": "Target blockchain"
                }
            },
            "required": ["proof_receipt", "chain"]
        }),
    }
}

/// List verifiers tool definition
fn list_verifiers_tool() -> ToolDefinition {
    ToolDefinition {
        name: "list_verifiers".to_string(),
        description: "List available on-chain verifiers and their status".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {}
        }),
    }
}
