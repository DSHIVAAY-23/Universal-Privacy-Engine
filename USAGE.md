# Usage Guide

## Overview

This guide provides practical examples for using the Universal Privacy Engine. It covers running benchmarks, generating verifiers, fetching data, and building complete privacy-preserving applications.

---

## Installation

```bash
# Clone the repository
git clone https://github.com/your-org/universal-privacy-engine
cd universal-privacy-engine

# Build the workspace
cargo build --workspace --release

# Run tests
cargo test --workspace
```

---

## Running Benchmarks

### Basic Usage

```bash
# Run all benchmarks (Mock backend)
cargo run --release --bin benchmark

# View results
cat benchmarks.json
```

### Output

```json
{
  "benchmarks": [
    {
      "scenario": "Small",
      "prover": "TEE",
      "mode": "Mock",
      "time_ms": 200,
      "cycles": null,
      "ram_mb": 0,
      "timestamp": "2025-12-26T15:30:00Z",
      "input_size_bytes": 256,
      "output_verified": true
    }
  ]
}
```

### Enabling SP1 Benchmarks

```bash
# Build SP1 guest program
cd guest/rwa_compliance
cargo prove build

# Run benchmarks with SP1
cd ../..
cargo run --release --bin benchmark
```

---

## Generating Solidity Verifiers

### Basic Usage

```bash
cargo run --bin generate_verifier -- \
  --program-vkey 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --out-dir contracts/generated
```

### With Custom Output Directory

```bash
cargo run --bin generate_verifier -- \
  --program-vkey 0x0000000000000000000000000000000000000000000000000000000000000001 \
  --out-dir /tmp/verifiers \
  --force  # Overwrite existing files
```

### Output

```
═══════════════════════════════════════════════════════════
         VeriVault Verifier Generator
═══════════════════════════════════════════════════════════

Validating program verification key...
  ✓ VKey: 0x1234...abcd

Generating verifier contract...
  ✓ Contract written to: contracts/generated/UniversalVerifier.sol

═══════════════════════════════════════════════════════════
                   GENERATION COMPLETE
═══════════════════════════════════════════════════════════
```

---

## Fetching Data with HttpProvider

### Basic HTTP Fetch

```rust
use universal_privacy_engine_core::data_source::{HttpProvider, DataProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new();
    
    // Fetch entire JSON response
    let data = provider.fetch(
        "https://api.example.com/data",
        ""  // Empty query returns full response
    ).await?;
    
    println!("Fetched {} bytes", data.len());
    Ok(())
}
```

### JSON Path Selector

```rust
use universal_privacy_engine_core::data_source::{HttpProvider, DataProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new();
    
    // Extract specific field
    let balance = provider.fetch(
        "https://api.bank.com/account/123",
        "data.balance"  // JSON path: data.balance
    ).await?;
    
    // Extract array element
    let first_item = provider.fetch(
        "https://api.example.com/items",
        "items[0]"  // First element of items array
    ).await?;
    
    // Complex path
    let user_name = provider.fetch(
        "https://api.example.com/users",
        "data.users[2].name"  // Nested path with array
    ).await?;
    
    Ok(())
}
```

---

## Building ZK Inputs

### Combining Data with Secrets

```rust
use universal_privacy_engine_core::data_source::ZkInputBuilder;

fn main() {
    let mut builder = ZkInputBuilder::new();
    
    // Add public data (visible in proof)
    builder.add_public_data(vec![1, 2, 3, 4]);
    
    // Add secrets (private in ZK proof)
    builder.add_secret(vec![5, 6, 7, 8]);
    
    // Build final input
    let zk_input = builder.build();
    
    println!("Input size: {} bytes", zk_input.len());
}
```

### Batch Operations

```rust
use universal_privacy_engine_core::data_source::ZkInputBuilder;

fn main() {
    let mut builder = ZkInputBuilder::new();
    
    // Add multiple public data fields
    builder.add_public_data_batch(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ]);
    
    // Add multiple secrets
    builder.add_secrets_batch(vec![
        vec![7, 8, 9],
        vec![10, 11, 12],
    ]);
    
    let zk_input = builder.build();
}
```

---

## Complete Privacy Pipeline

### Full Example: RWA Compliance

```rust
use universal_privacy_engine_core::{
    PrivacyEngine,
    data_source::{HttpProvider, DataProvider, ZkInputBuilder}
};
use universal_privacy_engine_tee::TeeProverStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Fetch institutional balance data
    let provider = HttpProvider::new();
    let balance_data = provider.fetch(
        "https://api.institution.com/balance",
        "data.balance"
    ).await?;
    
    // Step 2: Verify data authenticity (currently stub)
    assert!(provider.verify_tls_signature());
    
    // Step 3: Build ZK input
    let mut builder = ZkInputBuilder::new();
    builder
        .add_public_data(balance_data)
        .add_secret(user_private_key);
    
    let zk_input = builder.build();
    
    // Step 4: Generate proof
    let backend = TeeProverStub::new();
    let receipt = backend.prove(&zk_input)?;
    
    // Step 5: Verify proof
    let is_valid = backend.verify(&receipt)?;
    assert!(is_valid);
    
    println!("✓ Proof generated and verified successfully!");
    
    Ok(())
}
```

---

## Using Different Backends

### Mock Backend (Development)

```rust
use universal_privacy_engine_core::PrivacyEngine;
use universal_privacy_engine_tee::TeeProverStub;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = TeeProverStub::new();
    
    let input = b"test data";
    let receipt = backend.prove(input)?;
    
    let is_valid = backend.verify(&receipt)?;
    assert!(is_valid);
    
    Ok(())
}
```

### SP1 Backend (ZK-VM)

```rust
use universal_privacy_engine_core::PrivacyEngine;
use universal_privacy_engine_sp1::Sp1Backend;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load compiled guest program
    let elf = include_bytes!("../../guest/rwa_compliance/elf/riscv32im-succinct-zkvm-elf");
    
    let backend = Sp1Backend::new(elf);
    
    let input = b"test data";
    let receipt = backend.prove(input)?;
    
    let is_valid = backend.verify(&receipt)?;
    assert!(is_valid);
    
    Ok(())
}
```

---

## Deploying Generated Verifiers

### Step 1: Deploy Mock SP1 Verifier (Testing)

```solidity
// Deploy mock verifier
bytes32[] memory allowedPrograms = new bytes32[](1);
allowedPrograms[0] = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;

MockSP1Verifier mockVerifier = new MockSP1Verifier(allowedPrograms);
```

### Step 2: Deploy Universal Verifier

```solidity
// Deploy universal verifier
UniversalVerifier verifier = new UniversalVerifier(address(mockVerifier));
```

### Step 3: Verify Proofs On-Chain

```solidity
// Prepare proof data
bytes memory publicValues = abi.encode(balance, threshold);
bytes memory proof = /* ... generated by Privacy Engine ... */;

// Verify proof
bool success = verifier.verifyProof(publicValues, proof);
require(success, "Proof verification failed");
```

---

## Testing

### Run All Tests

```bash
cargo test --workspace
```

### Run Specific Module Tests

```bash
# Core module
cargo test -p universal-privacy-engine-core

# Mock adapter
cargo test -p universal-privacy-engine-tee

# Data ingestion
cargo test -p universal-privacy-engine-core data_source

# Verifier generator
cargo test --bin generate_verifier
```

### Run Tests with Output

```bash
cargo test --workspace -- --nocapture
```

---

## Common Patterns

### Error Handling

```rust
use universal_privacy_engine_core::{PrivacyEngine, PrivacyEngineError};

fn generate_proof(backend: &dyn PrivacyEngine, input: &[u8]) -> Result<(), PrivacyEngineError> {
    match backend.prove(input) {
        Ok(receipt) => {
            println!("Proof generated successfully");
            Ok(())
        }
        Err(PrivacyEngineError::InvalidInput(msg)) => {
            eprintln!("Invalid input: {}", msg);
            Err(PrivacyEngineError::InvalidInput(msg))
        }
        Err(e) => {
            eprintln!("Proof generation failed: {}", e);
            Err(e)
        }
    }
}
```

### Backend Polymorphism

```rust
use universal_privacy_engine_core::PrivacyEngine;
use universal_privacy_engine_tee::TeeProverStub;
use universal_privacy_engine_sp1::Sp1Backend;

fn verify_with_any_backend(
    backend: &dyn PrivacyEngine,
    input: &[u8]
) -> Result<bool, Box<dyn std::error::Error>> {
    let receipt = backend.prove(input)?;
    let is_valid = backend.verify(&receipt)?;
    Ok(is_valid)
}

fn main() {
    let tee_backend = TeeProverStub::new();
    let sp1_backend = Sp1Backend::new(elf);
    
    // Use same function with different backends
    verify_with_any_backend(&tee_backend, b"test")?;
    verify_with_any_backend(&sp1_backend, b"test")?;
}
```

---

## Performance Tips

### 1. Use Release Mode

```bash
# Always use --release for benchmarks and production
cargo run --release --bin benchmark
```

### 2. Reuse Builders

```rust
let mut builder = ZkInputBuilder::new();

for data in data_items {
    builder.clear();  // Reuse builder
    builder.add_public_data(data);
    let input = builder.build();
    // ... generate proof
}
```

### 3. Parallel Proof Generation

```rust
use rayon::prelude::*;

let receipts: Vec<_> = inputs
    .par_iter()
    .map(|input| backend.prove(input))
    .collect();
```

---

## Troubleshooting

### Issue: "VKey must be 32 bytes"

```bash
# Ensure VKey is exactly 64 hex characters (32 bytes)
cargo run --bin generate_verifier -- \
  --program-vkey 0x0000000000000000000000000000000000000000000000000000000000000001
```

### Issue: "SP1 benchmarks skipped"

```bash
# Build the guest program first
cd guest/rwa_compliance
cargo prove build
cd ../..
cargo run --release --bin benchmark
```

### Issue: "Field not found in JSON"

```rust
// Check JSON structure first
let data = provider.fetch("https://api.example.com", "").await?;
println!("{}", String::from_utf8_lossy(&data));

// Then use correct path
let field = provider.fetch("https://api.example.com", "correct.path").await?;
```

---

## Next Steps

1. **Read [ARCHITECTURE.md](ARCHITECTURE.md)** - Understand the hexagonal design
2. **Read [TRUST_MODEL.md](TRUST_MODEL.md)** - Understand security assumptions
3. **Read [DELIVERABLES.md](DELIVERABLES.md)** - See what's complete and what's pending
4. **Experiment** - Build your own privacy-preserving application!

---

## Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/your-org/universal-privacy-engine/issues)
- **Documentation**: [Full technical docs](https://docs.example.com)
- **Email**: [your-email@example.com]
