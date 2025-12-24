//! # Structured Extractor
//!
//! LLM-based extraction of RWA claims from unstructured data.
//! Supports PDF text, JSON, CSV, and API responses.

use crate::rwa::RwaClaim;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Data source types supported by the extractor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    /// Plain text (e.g., from PDF extraction)
    Text(String),
    
    /// JSON document
    Json(serde_json::Value),
    
    /// CSV data
    Csv(String),
    
    /// API response
    Api { url: String, response: String },
}

/// Result of extraction with confidence scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    /// The extracted RWA claim
    pub claim: RwaClaim,
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Hash of the source data
    pub source_hash: [u8; 32],
    
    /// Warnings or issues encountered
    pub warnings: Vec<String>,
    
    /// Extracted metadata
    pub metadata: HashMap<String, String>,
}

/// Structured extractor using LLM for parsing
pub struct StructuredExtractor {
    /// Prompt template for extraction
    prompt_template: String,
    
    /// Few-shot examples for better accuracy
    examples: Vec<(String, String)>,
}

impl StructuredExtractor {
    /// Create a new structured extractor
    pub fn new() -> Self {
        Self {
            prompt_template: Self::default_prompt_template(),
            examples: Self::default_examples(),
        }
    }

    /// Extract RWA claim from data source
    ///
    /// This method:
    /// 1. Sanitizes the input data
    /// 2. Builds a prompt with schema
    /// 3. Calls LLM for structured extraction (placeholder)
    /// 4. Validates the output
    /// 5. Returns with confidence score
    pub fn extract(&self, source: DataSource) -> Result<ExtractionResult, ExtractionError> {
        // Convert source to text
        let raw_text = match source {
            DataSource::Text(text) => text,
            DataSource::Json(json) => serde_json::to_string_pretty(&json)
                .map_err(|e| ExtractionError::InvalidInput(e.to_string()))?,
            DataSource::Csv(csv) => csv,
            DataSource::Api { response, .. } => response,
        };

        // Sanitize input (remove PII)
        let sanitized = self.sanitize_input(&raw_text);

        // Hash the source data
        let source_hash = self.hash_source(&raw_text);

        // Extract structured data (placeholder - would call LLM here)
        let extracted_data = self.extract_with_llm(&sanitized)?;

        // Parse into RwaClaim
        let claim = self.parse_claim(&extracted_data)?;

        // Calculate confidence
        let confidence = self.calculate_confidence(&extracted_data);

        // Collect warnings
        let warnings = self.collect_warnings(&extracted_data);

        Ok(ExtractionResult {
            claim,
            confidence,
            source_hash,
            warnings,
            metadata: extracted_data.metadata,
        })
    }

    /// Sanitize input to remove PII
    fn sanitize_input(&self, text: &str) -> String {
        let mut sanitized = text.to_string();

        // Remove SSN patterns (XXX-XX-XXXX)
        sanitized = regex::Regex::new(r"\d{3}-\d{2}-\d{4}")
            .unwrap()
            .replace_all(&sanitized, "[SSN REDACTED]")
            .to_string();

        // Remove account numbers (8+ digits)
        sanitized = regex::Regex::new(r"(?i)account\s*#?\s*(\d{8,})")
            .unwrap()
            .replace_all(&sanitized, "Account [REDACTED]")
            .to_string();

        // Remove credit card numbers
        sanitized = regex::Regex::new(r"\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}")
            .unwrap()
            .replace_all(&sanitized, "[CARD REDACTED]")
            .to_string();

        sanitized
    }

    /// Hash source data for audit trail
    fn hash_source(&self, text: &str) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hasher.finalize().into()
    }

    /// Extract structured data using LLM (placeholder)
    fn extract_with_llm(&self, sanitized_text: &str) -> Result<ExtractedData, ExtractionError> {
        // TODO: Integrate actual LLM SDK
        // For now, use simple regex-based extraction as placeholder
        
        let balance = self.extract_balance(sanitized_text)?;
        let institution = self.extract_institution(sanitized_text);
        let date = self.extract_date(sanitized_text);

        Ok(ExtractedData {
            balance,
            institution,
            date,
            metadata: HashMap::new(),
        })
    }

    /// Extract balance from text (simple regex)
    fn extract_balance(&self, text: &str) -> Result<u64, ExtractionError> {
        // Look for patterns like "$1,234.56" or "Balance: 1234.56"
        let re = regex::Regex::new(r"(?i)(?:balance|total|amount)[\s:$]*([0-9,]+\.?\d{0,2})")
            .unwrap();

        if let Some(caps) = re.captures(text) {
            let balance_str = caps.get(1)
                .ok_or(ExtractionError::BalanceNotFound)?
                .as_str()
                .replace(",", "");
            
            let balance_dollars: f64 = balance_str.parse()
                .map_err(|_| ExtractionError::InvalidBalance)?;
            
            // Convert to cents
            Ok((balance_dollars * 100.0) as u64)
        } else {
            Err(ExtractionError::BalanceNotFound)
        }
    }

    /// Extract institution name
    fn extract_institution(&self, text: &str) -> Option<String> {
        // Look for known bank names
        let banks = ["Chase", "Bank of America", "Wells Fargo", "Citi", "Goldman Sachs"];
        
        for bank in &banks {
            if text.contains(bank) {
                return Some(bank.to_string());
            }
        }
        
        None
    }

    /// Extract date
    fn extract_date(&self, text: &str) -> Option<String> {
        let re = regex::Regex::new(r"\d{4}-\d{2}-\d{2}|\d{2}/\d{2}/\d{4}")
            .unwrap();
        
        re.find(text).map(|m| m.as_str().to_string())
    }

    /// Parse extracted data into RwaClaim
    fn parse_claim(&self, data: &ExtractedData) -> Result<RwaClaim, ExtractionError> {
        // For now, create a placeholder claim
        // In production, this would use real institutional signatures
        
        Ok(RwaClaim::new(
            [1u8; 32],  // Placeholder institutional pubkey
            data.balance,
            data.balance / 2,  // Placeholder threshold (50% of balance)
            [2u8; 64],  // Placeholder signature
        ))
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, data: &ExtractedData) -> f32 {
        let mut confidence = 1.0;

        // Reduce confidence if institution not found
        if data.institution.is_none() {
            confidence *= 0.8;
        }

        // Reduce confidence if date not found
        if data.date.is_none() {
            confidence *= 0.9;
        }

        confidence
    }

    /// Collect warnings
    fn collect_warnings(&self, data: &ExtractedData) -> Vec<String> {
        let mut warnings = Vec::new();

        if data.institution.is_none() {
            warnings.push("Institution name not found in document".to_string());
        }

        if data.date.is_none() {
            warnings.push("Statement date not found".to_string());
        }

        warnings
    }

    /// Default prompt template
    fn default_prompt_template() -> String {
        r#"
You are a financial data extraction assistant. Extract the following information from the bank statement:

REQUIRED FIELDS:
- balance: Total account balance (numeric, in cents)
- institution: Name of the financial institution
- date: Statement date (YYYY-MM-DD)

INPUT DATA:
{input}

OUTPUT FORMAT (JSON):
{
  "balance": number,
  "institution": "string",
  "date": "YYYY-MM-DD"
}

RULES:
1. Balance must be in cents (multiply dollars by 100)
2. Only extract factual data, no estimates
3. If a field is unclear, set it to null
        "#.to_string()
    }

    /// Default few-shot examples
    fn default_examples() -> Vec<(String, String)> {
        vec![
            (
                "Chase Bank Statement\nAccount Balance: $1,234.56\nDate: 2024-01-15".to_string(),
                r#"{"balance": 123456, "institution": "Chase Bank", "date": "2024-01-15"}"#.to_string()
            ),
        ]
    }
}

impl Default for StructuredExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Intermediate extracted data
#[derive(Debug)]
struct ExtractedData {
    balance: u64,
    institution: Option<String>,
    date: Option<String>,
    metadata: HashMap<String, String>,
}

/// Extraction errors
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Balance not found in document")]
    BalanceNotFound,
    
    #[error("Invalid balance format")]
    InvalidBalance,
    
    #[error("LLM extraction failed: {0}")]
    LlmError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_ssn() {
        let extractor = StructuredExtractor::new();
        let text = "SSN: 123-45-6789";
        let sanitized = extractor.sanitize_input(text);
        assert!(!sanitized.contains("123-45-6789"));
        assert!(sanitized.contains("[SSN REDACTED]"));
    }

    #[test]
    fn test_extract_balance() {
        let extractor = StructuredExtractor::new();
        let text = "Account Balance: $1,234.56";
        let balance = extractor.extract_balance(text).unwrap();
        assert_eq!(balance, 123456);
    }

    #[test]
    fn test_extract_from_text() {
        let extractor = StructuredExtractor::new();
        let source = DataSource::Text(
            "Chase Bank\nAccount Balance: $50,000.00\nDate: 2024-01-15".to_string()
        );
        
        let result = extractor.extract(source).unwrap();
        assert_eq!(result.claim.balance, 5000000);
        assert!(result.confidence > 0.7);
    }
}
