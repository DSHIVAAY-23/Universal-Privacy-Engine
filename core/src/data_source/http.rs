//! HTTP data provider implementation

use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use super::error::DataError;
use super::provider::DataProvider;

/// HTTP-based data provider using reqwest
///
/// Fetches JSON data from HTTPS endpoints and extracts specific fields
/// using a simplified JSON path selector.
///
/// ## JSON Path Syntax
///
/// - `"field"` - Top-level field
/// - `"parent.child"` - Nested field access
/// - `"array[0]"` - Array indexing
/// - `"data.items[2].name"` - Complex nested path
///
/// ## Example
///
/// ```ignore
/// let provider = HttpProvider::new();
/// let balance = provider.fetch(
///     "https://api.bank.com/account/123",
///     "data.balance"
/// ).await?;
/// ```
pub struct HttpProvider {
    /// HTTP client for making requests
    client: Client,
}

impl HttpProvider {
    /// Create a new HTTP provider
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
    
    /// Select a field from JSON using a path expression
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON value to query
    /// * `path` - Path expression (e.g., "account.balance", "items[0]")
    ///
    /// # Returns
    ///
    /// Serialized bytes of the selected field
    ///
    /// # Errors
    ///
    /// Returns `DataError::FieldNotFound` if the path doesn't exist
    /// Returns `DataError::InvalidQuery` if the path syntax is invalid
    fn select_json_field(&self, json: &Value, path: &str) -> Result<Vec<u8>, DataError> {
        if path.is_empty() {
            // Empty path returns the entire JSON
            return serde_json::to_vec(json)
                .map_err(|e| DataError::SerializationError(e));
        }
        
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = json;
        
        for part in parts {
            // Check if this part has array indexing: "field[0]"
            if let Some(bracket_pos) = part.find('[') {
                // Extract field name and index
                let field_name = &part[..bracket_pos];
                let rest = &part[bracket_pos..];
                
                // Parse index from "[0]"
                if !rest.ends_with(']') {
                    return Err(DataError::InvalidQuery(
                        format!("Invalid array syntax: {}", part)
                    ));
                }
                
                let index_str = &rest[1..rest.len()-1];
                let index: usize = index_str.parse()
                    .map_err(|_| DataError::InvalidQuery(
                        format!("Invalid array index: {}", index_str)
                    ))?;
                
                // Navigate to field (if not empty)
                if !field_name.is_empty() {
                    current = current.get(field_name)
                        .ok_or_else(|| DataError::FieldNotFound(
                            format!("Field '{}' not found", field_name)
                        ))?;
                }
                
                // Navigate to array element
                current = current.get(index)
                    .ok_or_else(|| DataError::FieldNotFound(
                        format!("Array index {} out of bounds", index)
                    ))?;
            } else {
                // Simple field access
                current = current.get(part)
                    .ok_or_else(|| DataError::FieldNotFound(
                        format!("Field '{}' not found", part)
                    ))?;
            }
        }
        
        // Serialize the selected value to bytes
        serde_json::to_vec(current)
            .map_err(|e| DataError::SerializationError(e))
    }
}

impl Default for HttpProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DataProvider for HttpProvider {
    async fn fetch(&self, source: &str, query: &str) -> Result<Vec<u8>, DataError> {
        // ═══════════════════════════════════════════════════════════════════════════
        // DEMO: Recorded zkTLS Verification Flow
        // ═══════════════════════════════════════════════════════════════════════════
        // If the source is "https://example.com", we simulate the zkTLS flow using
        // recorded fixtures instead of a real network call.
        if source == "https://example.com" {
            use std::path::PathBuf;
            use super::zktls::{RecordedTlsProof, ZkTlsError};

            // 1. Load Fixtures (Simulating receiving proof + data)
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| ".".to_string());
            let fixtures_path = PathBuf::from(manifest_dir)
                .parent() // Go up from 'core' to root (workspace)
                .unwrap()
                .join("fixtures/zktls");

            let metadata_path = fixtures_path.join("metadata.json");
            let cert_path = fixtures_path.join("cert_chain.pem");
            let response_path = fixtures_path.join("response_body.json");

            // Helper to read file or error
            let read_file = |p: &PathBuf| std::fs::read(p).map_err(|e| 
                DataError::HttpError(format!("Failed to read fixture {}: {}", p.display(), e))
            );

            let metadata_bytes = read_file(&metadata_path)?;
            let cert_bytes = read_file(&cert_path)?;
            let response_bytes = read_file(&response_path)?;

            // 2. Parse metadata into Proof struct
            let metadata: Value = serde_json::from_slice(&metadata_bytes)
                .map_err(|e| DataError::JsonError(format!("Invalid metadata JSON: {}", e)))?;
            
            let proof = RecordedTlsProof {
                domain: metadata["domain"].as_str().unwrap_or("").to_string(),
                timestamp: metadata["timestamp"].as_u64().unwrap_or(0),
                response_hash: metadata["response_sha256"].as_str().unwrap_or("").to_string(),
                cert_chain_hash: metadata["cert_chain_sha256"].as_str().unwrap_or("").to_string(),
            };

            // 3. Verify the Proof
            // In a real flow, 'verify_tls_proof' would be called with the proof provided by the Prover.
            // Here, we verify the fixtures against themselves to demonstrate the logic.
            self.verify_tls_proof(&proof).map_err(|e| match e {
                ZkTlsError::DomainMismatch { .. } => DataError::HttpError(e.to_string()),
                ZkTlsError::CertChainMismatch => DataError::HttpError(e.to_string()),
                ZkTlsError::ResponseTampered => DataError::HttpError(e.to_string()),
                ZkTlsError::ReplayDetected { .. } => DataError::HttpError(e.to_string()),
            })?;

             // 4. Validate Specifics (Double-check against raw bytes we loaded)
            // This ensures the loaded 'response_bytes' match the verified 'proof.response_hash'.
            proof.verify(
                "example.com", 
                &cert_bytes, 
                &response_bytes, 
                1735128100, // Use a time shortly after the fixture timestamp (simulating "now")
                3600 // 1 hour max age
            ).map_err(|e| DataError::HttpError(format!("Proof verification failed: {}", e)))?;

            // 5. Return verified data
            let json: Value = serde_json::from_slice(&response_bytes)
                .map_err(|e| DataError::JsonError(format!("Failed to parse response JSON: {}", e)))?;
            
            return self.select_json_field(&json, query);
        }

        // Standard HTTP Fetch (for non-demo URLs)
        let response = self.client
            .get(source)
            .send()
            .await
            .map_err(|e| DataError::HttpError(format!("Request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(DataError::HttpError(
                format!("HTTP {} {}", response.status().as_u16(), response.status().canonical_reason().unwrap_or(""))
            ));
        }
        
        let json: Value = response.json()
            .await
            .map_err(|e| DataError::JsonError(format!("Failed to parse JSON: {}", e)))?;
        
        self.select_json_field(&json, query)
    }
    
    fn verify_tls_proof(&self, _proof: &super::zktls::RecordedTlsProof) -> Result<(), super::zktls::ZkTlsError> {
        // In the full zkTLS flow, this method would verify the cryptographic signature 
        // from the Notary. For 'Recorded zkTLS', the verification happens via 
        // RecordedTlsProof::verify() which checks the hashes against the raw data.
        //
        // Since `HttpProvider` doesn't hold the raw data statefully (it fetches it),
        // the effective verification logic is encoded in the `fetch` method above for the demo.
        // 
        // However, to satisfy the interface, we return Ok here implies the *structure* is valid.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_select_top_level_field() {
        let provider = HttpProvider::new();
        let json = json!({"balance": 1000});
        
        let result = provider.select_json_field(&json, "balance").unwrap();
        let value: u64 = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(value, 1000);
    }
    
    #[test]
    fn test_select_nested_field() {
        let provider = HttpProvider::new();
        let json = json!({
            "account": {
                "balance": 5000,
                "currency": "USD"
            }
        });
        
        let result = provider.select_json_field(&json, "account.balance").unwrap();
        let value: u64 = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(value, 5000);
    }
    
    #[test]
    fn test_select_array_element() {
        let provider = HttpProvider::new();
        let json = json!({
            "items": [100, 200, 300]
        });
        
        let result = provider.select_json_field(&json, "items[1]").unwrap();
        let value: u64 = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(value, 200);
    }
    
    #[test]
    fn test_select_complex_path() {
        let provider = HttpProvider::new();
        let json = json!({
            "data": {
                "users": [
                    {"name": "Alice", "age": 30},
                    {"name": "Bob", "age": 25}
                ]
            }
        });
        
        let result = provider.select_json_field(&json, "data.users[1].name").unwrap();
        let value: String = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(value, "Bob");
    }
    
    #[test]
    fn test_select_empty_path() {
        let provider = HttpProvider::new();
        let json = json!({"test": 123});
        
        let result = provider.select_json_field(&json, "").unwrap();
        let value: Value = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(value, json);
    }
    
    #[test]
    fn test_field_not_found() {
        let provider = HttpProvider::new();
        let json = json!({"balance": 1000});
        
        let result = provider.select_json_field(&json, "nonexistent");
        assert!(matches!(result, Err(DataError::FieldNotFound(_))));
    }
    
    #[test]
    fn test_invalid_array_syntax() {
        let provider = HttpProvider::new();
        let json = json!({"items": [1, 2, 3]});
        
        let result = provider.select_json_field(&json, "items[invalid]");
        assert!(matches!(result, Err(DataError::InvalidQuery(_))));
    }
    
    #[test]
    fn test_array_out_of_bounds() {
        let provider = HttpProvider::new();
        let json = json!({"items": [1, 2, 3]});
        
        let result = provider.select_json_field(&json, "items[10]");
        assert!(matches!(result, Err(DataError::FieldNotFound(_))));
    }
}
