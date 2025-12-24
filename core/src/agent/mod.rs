//! # Agent Module
//!
//! This module provides intelligent automation for ZK proof generation.
//! It includes LLM-based data extraction, schema validation, and orchestration.

pub mod extractor;
pub mod validator;
pub mod orchestrator;

pub use extractor::{StructuredExtractor, ExtractionResult, DataSource};
pub use validator::{SchemaValidator, ValidationResult};
pub use orchestrator::{ChainOrchestrator, SubmissionResult};
