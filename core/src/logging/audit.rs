//! # ZK Audit Trail
//!
//! Verifiable logging of all agent actions for compliance and debugging.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// Agent action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentAction {
    ExtractClaim,
    GenerateProof,
    SubmitToChain,
    VerifyProof,
    ExportVerifier,
}

/// Single audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unix timestamp
    pub timestamp: i64,
    
    /// Action performed
    pub action: AgentAction,
    
    /// Hash of input data
    pub input_hash: [u8; 32],
    
    /// Hash of output data
    pub output_hash: [u8; 32],
    
    /// Hash of decision logic/code
    pub decision_logic_hash: [u8; 32],
    
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    
    /// Hash of previous entry (blockchain-like)
    pub previous_hash: [u8; 32],
    
    /// Nonce for uniqueness
    pub nonce: u64,
}

impl AuditEntry {
    /// Create a new audit entry
    pub fn new(
        action: AgentAction,
        input: &[u8],
        output: &[u8],
        decision_logic: &[u8],
        confidence: f32,
        previous_hash: [u8; 32],
        nonce: u64,
    ) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            action,
            input_hash: Self::hash(input),
            output_hash: Self::hash(output),
            decision_logic_hash: Self::hash(decision_logic),
            confidence,
            previous_hash,
            nonce,
        }
    }

    /// Hash data using SHA256
    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Compute hash of this entry
    pub fn compute_hash(&self) -> [u8; 32] {
        let serialized = serde_json::to_vec(self).unwrap();
        Self::hash(&serialized)
    }
}

/// ZK Audit Trail - verifiable log of agent decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkAuditTrail {
    /// All audit entries
    pub entries: Vec<AuditEntry>,
    
    /// Hash of the entire trail
    pub trail_hash: [u8; 32],
    
    /// Trail creation timestamp
    pub created_at: i64,
}

impl ZkAuditTrail {
    /// Create a new audit trail
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            trail_hash: [0u8; 32],
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }

    /// Add an entry to the trail
    pub fn add_entry(
        &mut self,
        action: AgentAction,
        input: &[u8],
        output: &[u8],
        decision_logic: &[u8],
        confidence: f32,
    ) {
        let previous_hash = self.entries.last()
            .map(|e| e.compute_hash())
            .unwrap_or([0u8; 32]);

        let nonce = self.entries.len() as u64;

        let entry = AuditEntry::new(
            action,
            input,
            output,
            decision_logic,
            confidence,
            previous_hash,
            nonce,
        );

        self.entries.push(entry);
        self.update_trail_hash();
    }

    /// Update the trail hash
    fn update_trail_hash(&mut self) {
        let serialized = serde_json::to_vec(&self.entries).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        self.trail_hash = hasher.finalize().into();
    }

    /// Verify trail integrity
    pub fn verify_integrity(&self) -> bool {
        // Check each entry's previous_hash matches the previous entry's hash
        for i in 1..self.entries.len() {
            let prev_hash = self.entries[i - 1].compute_hash();
            if self.entries[i].previous_hash != prev_hash {
                return false;
            }
        }

        // Verify trail hash
        let serialized = serde_json::to_vec(&self.entries).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let computed_hash: [u8; 32] = hasher.finalize().into();

        computed_hash == self.trail_hash
    }

    /// Export trail as JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get total entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if trail is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ZkAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_trail_creation() {
        let trail = ZkAuditTrail::new();
        assert_eq!(trail.len(), 0);
        assert!(trail.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let mut trail = ZkAuditTrail::new();
        
        trail.add_entry(
            AgentAction::ExtractClaim,
            b"input data",
            b"output data",
            b"decision logic",
            0.95,
        );

        assert_eq!(trail.len(), 1);
        assert!(!trail.is_empty());
    }

    #[test]
    fn test_trail_integrity() {
        let mut trail = ZkAuditTrail::new();
        
        trail.add_entry(
            AgentAction::ExtractClaim,
            b"input1",
            b"output1",
            b"logic1",
            0.9,
        );

        trail.add_entry(
            AgentAction::GenerateProof,
            b"input2",
            b"output2",
            b"logic2",
            0.95,
        );

        assert!(trail.verify_integrity());
    }

    #[test]
    fn test_tampered_trail() {
        let mut trail = ZkAuditTrail::new();
        
        trail.add_entry(
            AgentAction::ExtractClaim,
            b"input1",
            b"output1",
            b"logic1",
            0.9,
        );

        trail.add_entry(
            AgentAction::GenerateProof,
            b"input2",
            b"output2",
            b"logic2",
            0.95,
        );

        // Tamper with an entry
        trail.entries[0].confidence = 0.5;

        // Integrity check should fail
        assert!(!trail.verify_integrity());
    }

    #[test]
    fn test_export_json() {
        let mut trail = ZkAuditTrail::new();
        trail.add_entry(
            AgentAction::ExtractClaim,
            b"input",
            b"output",
            b"logic",
            0.9,
        );

        let json = trail.export_json().unwrap();
        assert!(json.contains("ExtractClaim"));
    }
}
