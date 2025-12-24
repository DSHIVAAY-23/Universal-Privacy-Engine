//! # Logging Module
//!
//! Verifiable audit trails for agent decisions.

pub mod audit;

pub use audit::{ZkAuditTrail, AuditEntry, AgentAction};
