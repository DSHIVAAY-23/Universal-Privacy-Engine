//! # Schema Validator
//!
//! Validates extracted data against RwaClaim schema to prevent hallucinations.

use crate::rwa::RwaClaim;
use serde::{Deserialize, Serialize};

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Schema validator for RwaClaim
pub struct SchemaValidator;

impl SchemaValidator {
    /// Validate an RwaClaim
    pub fn validate(claim: &RwaClaim) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate balance is non-zero
        if claim.balance == 0 {
            errors.push("Balance cannot be zero".to_string());
        }

        // Validate threshold is reasonable
        if claim.threshold > claim.balance {
            errors.push("Threshold cannot exceed balance".to_string());
        }

        // Validate pubkey is not all zeros
        if claim.institutional_pubkey == [0u8; 32] {
            warnings.push("Institutional pubkey appears to be placeholder".to_string());
        }

        // Validate signature is not all zeros
        if claim.signature == [0u8; 64] {
            warnings.push("Signature appears to be placeholder".to_string());
        }

        // Check for suspiciously large balances (> $1B)
        if claim.balance > 100_000_000_000 {
            warnings.push("Balance exceeds $1 billion - please verify".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_claim() {
        let claim = RwaClaim::new([1u8; 32], 1000000, 500000, [2u8; 64]);
        let result = SchemaValidator::validate(&claim);
        assert!(result.is_valid);
    }

    #[test]
    fn test_zero_balance() {
        let claim = RwaClaim::new([1u8; 32], 0, 500000, [2u8; 64]);
        let result = SchemaValidator::validate(&claim);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("Balance cannot be zero")));
    }

    #[test]
    fn test_threshold_exceeds_balance() {
        let claim = RwaClaim::new([1u8; 32], 100000, 500000, [2u8; 64]);
        let result = SchemaValidator::validate(&claim);
        assert!(!result.is_valid);
    }
}
