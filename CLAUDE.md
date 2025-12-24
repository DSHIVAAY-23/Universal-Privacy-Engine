# CLAUDE.md - AI Context for Universal Privacy Engine

## Project Overview

The **Universal Privacy Engine** is a multi-chain RWA compliance layer built with:
- **Rust** for systems programming
- **SP1** for zero-knowledge proofs
- **MCP** for agentic automation
- **Multi-chain verifiers** (Solana/Stellar/Mantra)

This document provides context for AI assistants (Claude, Cursor, etc.) working on this codebase.

---

## Build Commands

### Quick Reference

```bash
# Check workspace (fast)
cargo check --workspace

# Build release
cargo build --release

# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p universal-privacy-engine-core
cargo test -p universal-privacy-engine-sp1
cargo test -p universal-privacy-engine-cli

# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Start MCP server
cargo run --bin upe -- mcp-server
```

### Build Troubleshooting

**Issue**: Compilation errors in guest program
```bash
# Solution: Check SP1 version
sp1 --version  # Should be 3.4+

# Rebuild guest
cd guest/rwa_compliance
cargo build --release
```

**Issue**: Missing dependencies
```bash
# Solution: Update Cargo.lock
cargo update
cargo build
```

---

## Common Testing Patterns

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = function_under_test();
        assert_eq!(result, expected_value);
    }
}
```

### Integration Tests

```rust
// In adapters/sp1/tests/integration_test.rs
use universal_privacy_engine_core::rwa::RwaClaim;
use universal_privacy_engine_sp1::ProvingMode;

#[test]
fn test_rwa_claim_creation() {
    let claim = RwaClaim::new([1u8; 32], 1000000, 500000, [2u8; 64]);
    assert_eq!(claim.balance, 1000000);
}
```

### Running Specific Tests

```bash
# Run tests matching pattern
cargo test extract

# Run tests in specific file
cargo test --test integration_test

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_rwa_claim_creation -- --exact
```

---

## "Vibe Coding" Rules

### 1. Always Use SP1 Precompiles

**❌ Wrong**:
```rust
// Don't use pure Rust crypto in guest
use ed25519_dalek::Verifier;
let valid = verifier.verify(&message, &signature).is_ok();
```

**✅ Correct**:
```rust
// Use SP1 precompile (10-100x faster)
let valid = sp1_zkvm::lib::verify_ed25519(&signature, &pubkey, &message);
```

**Why**: Precompiles use optimized ZK circuits, dramatically reducing proof generation time.

### 2. Borsh for zkVM, Serde for Host

**❌ Wrong**:
```rust
// Don't use serde in guest program
use serde::{Serialize, Deserialize};
let claim: RwaClaim = serde_json::from_slice(&data)?;
```

**✅ Correct**:
```rust
// Use Borsh in guest (deterministic)
use borsh::BorshDeserialize;
let claim = RwaClaim::try_from_slice(&sp1_zkvm::io::read_vec())?;
```

**Why**: Borsh is deterministic and has minimal overhead in constrained zkVM environments.

### 3. Keep Guest Programs Simple

**❌ Wrong**:
```rust
// Don't add complex logic in guest
fn main() {
    let data = fetch_from_api("https://...");  // ❌ No network in zkVM
    let result = complex_ml_model(data);       // ❌ Too expensive to prove
}
```

**✅ Correct**:
```rust
// Keep guest focused on verification
fn main() {
    let claim = RwaClaim::try_from_slice(&sp1_zkvm::io::read_vec())?;
    assert!(claim.balance >= claim.threshold);  // Simple assertion
    sp1_zkvm::io::commit(&claim.institutional_pubkey);
}
```

**Why**: Every instruction in the guest becomes part of the proof. Keep it minimal.

### 4. Sanitize Before LLM

**❌ Wrong**:
```rust
// Don't send raw PII to LLM
let response = llm_client.complete(raw_bank_statement).await?;
```

**✅ Correct**:
```rust
// Sanitize first
let sanitized = sanitize_pii(raw_bank_statement);
let response = llm_client.complete(sanitized).await?;
```

**Why**: Privacy-first design. PII should never leave the local environment.

### 5. Validate Everything

**❌ Wrong**:
```rust
// Don't trust extracted data
let claim = extractor.extract(source)?;
backend.prove(&claim)?;  // ❌ No validation
```

**✅ Correct**:
```rust
// Always validate
let claim = extractor.extract(source)?;
let validation = SchemaValidator::validate(&claim);
if !validation.is_valid {
    return Err(Error::InvalidClaim(validation.errors));
}
backend.prove(&claim)?;
```

**Why**: LLM extraction can hallucinate. Validate before proving.

---

## Code Patterns

### Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Proving failed: {0}")]
    ProvingFailed(String),
}

// Usage
fn my_function() -> Result<T, MyError> {
    let data = parse_input()
        .map_err(|e| MyError::InvalidInput(e.to_string()))?;
    Ok(data)
}
```

### Async Functions

```rust
// Use tokio for async
#[tokio::main]
async fn main() -> Result<()> {
    let result = async_operation().await?;
    Ok(())
}

// Async in tests
#[tokio::test]
async fn test_async() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Serialization

```rust
// Borsh (zkVM)
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyStruct {
    pub field: u64,
}

// Serde (host)
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    pub field: u64,
}
```

---

## Architecture Patterns

### Trait-Based Abstraction

```rust
// Define trait in core
pub trait PrivacyEngine {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt>;
    fn verify(&self, receipt: &ProofReceipt) -> Result<bool>;
}

// Implement in adapter
impl PrivacyEngine for Sp1Backend {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt> {
        // SP1-specific implementation
    }
}
```

### Module Organization

```
crate/
├── src/
│   ├── lib.rs           # Public API
│   ├── module1/
│   │   ├── mod.rs       # Module exports
│   │   ├── submodule.rs
│   │   └── tests.rs     # Module tests
│   └── module2/
│       └── mod.rs
```

### Workspace Structure

```toml
# Root Cargo.toml
[workspace]
members = ["core", "adapters/sp1", "cli"]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }

# Member Cargo.toml
[dependencies]
serde.workspace = true
```

---

## Debugging Tips

### Print Debugging in Guest

```rust
// Use sp1_zkvm::io::hint for debugging
sp1_zkvm::io::hint(&format!("Debug: balance = {}", claim.balance));
```

### Verbose Test Output

```bash
# See all println! output
cargo test -- --nocapture

# Show test names
cargo test -- --test-threads=1
```

### Check Specific Warnings

```bash
# Show all warnings
cargo clippy --all-targets

# Fix automatically
cargo fix --allow-dirty
```

---

## Performance Optimization

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile binary
cargo flamegraph --bin upe -- prove --input <data>
```

### Benchmarking

```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
cargo criterion
```

### Memory Usage

```bash
# Check binary size
ls -lh target/release/upe

# Strip binary
strip target/release/upe
```

---

## Common Issues & Solutions

### Issue: "cannot find module or unlinked crate"

**Solution**:
```bash
# Add dependency to Cargo.toml
[dependencies]
missing_crate = "1.0"

# Or use workspace dependency
missing_crate.workspace = true
```

### Issue: "trait is not in scope"

**Solution**:
```rust
// Import trait
use crate::PrivacyEngine;

// Or use fully qualified syntax
<Sp1Backend as PrivacyEngine>::prove(&backend, input)
```

### Issue: "lifetime errors"

**Solution**:
```rust
// Use explicit lifetimes
fn function<'a>(data: &'a str) -> &'a str {
    data
}

// Or use owned types
fn function(data: String) -> String {
    data
}
```

---

## Git Workflow

### Commit Messages

```bash
# Format: <type>: <description>
git commit -m "feat: Add Groth16 proving pipeline"
git commit -m "fix: Resolve Ed25519 precompile API issue"
git commit -m "docs: Update README with benchmarks"
git commit -m "test: Add integration tests for MCP server"
```

### Branch Strategy

```bash
# Feature branches
git checkout -b feature/stellar-verifier

# Bug fixes
git checkout -b fix/audit-trail-integrity

# Documentation
git checkout -b docs/api-reference
```

---

## Documentation Standards

### Code Comments

```rust
/// Brief description of function.
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Description of return value
///
/// # Example
///
/// ```
/// let result = function(arg1, arg2);
/// ```
pub fn function(param1: Type1, param2: Type2) -> ReturnType {
    // Implementation
}
```

### Module Documentation

```rust
//! # Module Name
//!
//! Brief description of module purpose.
//!
//! ## Features
//!
//! - Feature 1
//! - Feature 2
//!
//! ## Example
//!
//! ```
//! use crate::module::Function;
//! let result = Function::new();
//! ```
```

---

## Security Checklist

Before committing code, verify:

- [ ] No hardcoded secrets or API keys
- [ ] PII is sanitized before LLM
- [ ] Input validation on all external data
- [ ] Error messages don't leak sensitive info
- [ ] Audit trail logs all critical operations
- [ ] Tests cover security-critical paths

---

## Quick Reference

### File Locations

- **Core Types**: `core/src/lib.rs`
- **RWA Types**: `core/src/rwa.rs`
- **SP1 Adapter**: `adapters/sp1/src/lib.rs`
- **Guest Program**: `guest/rwa_compliance/src/main.rs`
- **CLI**: `cli/src/main.rs`
- **MCP Server**: `cli/src/mcp/server.rs`
- **Solana Verifier**: `verifiers/solana/programs/rwa-verifier/src/lib.rs`
- **Stellar Verifier**: `verifiers/stellar/src/lib.rs`
- **Mantra Verifier**: `verifiers/mantra/src/lib.rs`

### Key Dependencies

- `sp1-sdk = "3.4"` - SP1 proving system
- `borsh = "1.5"` - Deterministic serialization
- `serde = "1.0"` - General serialization
- `tokio = "1.35"` - Async runtime
- `clap = "4.0"` - CLI parsing
- `thiserror = "1.0"` - Error handling

---

## AI Assistant Guidelines

When working on this codebase:

1. **Always run tests** after making changes
2. **Follow "Vibe Coding" rules** (precompiles, Borsh, etc.)
3. **Validate extracted data** before proving
4. **Keep guest programs minimal** (every instruction costs proof time)
5. **Document security-critical code** thoroughly
6. **Use workspace dependencies** for version consistency
7. **Check benchmarks** after performance-related changes

---

## Contact & Resources

- **Repository**: https://github.com/DSHIVAAY-23/Universal-Privacy-Engine
- **SP1 Docs**: https://docs.succinct.xyz/
- **Solana Anchor**: https://www.anchor-lang.com/
- **Stellar Soroban**: https://soroban.stellar.org/
- **CosmWasm**: https://docs.cosmwasm.com/

---

**Last Updated**: 2024-12-24
**Version**: Phase 5 (Production Documentation)
