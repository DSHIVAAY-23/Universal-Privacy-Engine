# Universal Privacy Engine (VeriVault Core)

> ⚠️ **Status: Alpha Research Prototype**  
> This is a research prototype demonstrating modular privacy-preserving computation infrastructure. Not production-ready. Seeking funding for TEE and zkTLS integration.

## Overview

The **Universal Privacy Engine** is a modular Rust framework for building privacy-preserving applications using zero-knowledge proofs (ZK-VMs) and Trusted Execution Environments (TEEs). It provides a unified abstraction layer that enables developers to swap proving backends without changing application logic.

**Current State**: Research prototype with working SP1 zkVM integration and mock TEE adapter. Demonstrates client-side proof generation with data ingestion from HTTPS sources.

**Target State**: Production-grade privacy infrastructure with hardware-backed TEEs (Intel SGX, AWS Nitro) and cryptographic data authenticity via zkTLS (TLSNotary/DECO).

---

## Key Features

### 🏗️ **Modular Core Architecture**
- **Hexagonal/Ports-and-Adapters** pattern for backend independence
- `PrivacyEngine` trait as the universal interface
- `ProofType` enum distinguishing ZK proofs from TEE attestations
- Swappable adapters: SP1 (ZK-VM), TEE Mock, future RISC0/Plonky2

### 📊 **Data Ingestion Layer**
- `DataProvider` trait for fetching external data
- `HttpProvider` with JSON path selector (jq-like syntax)
- `ZkInputBuilder` for combining public data with secrets
- zkTLS integration stubs (TLSNotary/DECO roadmap)

### ⚡ **Performance Benchmarking**
- Automated benchmark suite for proving backends
- Metrics: wall time, RAM usage, CPU cycles
- JSON output for grant proposals and optimization
- TEE vs SP1 comparison

### 🔐 **Smart Contract Generator**
- Auto-generates Solidity verifiers from templates
- CLI tool: `generate_verifier --program-vkey <HASH>`
- Wraps SP1 Groth16/PLONK verifiers with universal interface
- Mock verifier for development

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│              (CLI, API, Data Ingestion)                      │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
                    ┌───────────────┐
                    │ PrivacyEngine │  ◄── Port (Trait)
                    │     Trait     │
                    └───────┬───────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ Sp1Backend   │ │TeeProverStub │ │ Future: RISC0│
    │   (Adapter)  │ │  (Adapter)   │ │   (Adapter)  │
    └──────────────┘ └──────────────┘ └──────────────┘
         ZK-VM           TEE Mock         ZK-VM
```

---

## Quick Start

### Installation

```bash
git clone https://github.com/your-org/universal-privacy-engine
cd universal-privacy-engine
cargo build --workspace
```

### Full Privacy Pipeline

```rust
use universal_privacy_engine_core::{
    PrivacyEngine,
    data_source::{HttpProvider, DataProvider, ZkInputBuilder}
};
use universal_privacy_engine_tee::TeeProverStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Fetch data from HTTPS API
    let provider = HttpProvider::new();
    let balance = provider.fetch(
        "https://api.bank.com/account/123",
        "data.balance"  // JSON path selector
    ).await?;
    
    // 2. Verify data authenticity (currently stub)
    assert!(provider.verify_tls_signature());
    
    // 3. Build ZK input (combine data + secrets)
    let mut builder = ZkInputBuilder::new();
    builder
        .add_public_data(balance)
        .add_secret(user_private_key);
    
    let zk_input = builder.build();
    
    // 4. Generate proof (TEE or ZK-VM)
    let backend = TeeProverStub::new();
    let receipt = backend.prove(&zk_input)?;
    
    // 5. Verify proof
    let is_valid = backend.verify(&receipt)?;
    assert!(is_valid);
    
    Ok(())
}
```

### Run Benchmarks

```bash
cargo run --release --bin benchmark
cat benchmarks.json
```

### Generate Solidity Verifier

```bash
cargo run --bin generate_verifier -- \
  --program-vkey 0x1234567890abcdef... \
  --out-dir contracts/generated
```

---

## Project Structure

```
universal-privacy-engine/
├── core/                      # Core abstraction layer
│   ├── src/
│   │   ├── lib.rs            # PrivacyEngine trait, ProofType enum
│   │   ├── data_source/      # Data ingestion (HttpProvider, ZkInputBuilder)
│   │   ├── rwa/              # Real-World Asset types
│   │   └── agent/            # Automation infrastructure
│   └── Cargo.toml
├── adapters/
│   ├── sp1/                  # SP1 zkVM adapter
│   └── tee/                  # TEE mock adapter (SGX/Nitro stub)
├── bin/                      # Benchmark binary
├── scripts/                  # Verifier generator
├── contracts/
│   ├── templates/            # Tera templates for Solidity
│   └── generated/            # Auto-generated verifiers
└── guest/                    # SP1 guest programs
```

---

## Current Limitations (Alpha Status)

### ⚠️ **Trust Model**
- **Client-side proving**: Prover runs on user's machine (no hardware isolation)
- **Mock TEE**: `TeeProverStub` simulates attestations without real SGX/Nitro
- **No zkTLS**: Data authenticity relies on trusting HTTPS endpoints

### ⚠️ **Performance**
- **SP1 proving time**: 30s-3min depending on workload (Mock mode)
- **TEE mock**: Simulates 200ms delay, no real cryptography
- **Not optimized**: Research prototype, not production-tuned

### ⚠️ **Deployment**
- **No production deployments**: Contracts are templates, not audited
- **Mock verifiers**: Development-only, not cryptographically secure
- **Limited chain support**: EVM focus, Solana/Stellar adapters incomplete

---

## Roadmap to Production

### Phase 1: TEE Integration (Seeking Funding)
- [ ] Intel SGX adapter with DCAP attestation
- [ ] AWS Nitro Enclaves integration
- [ ] Azure Confidential Computing support
- [ ] Hardware-backed key management

### Phase 2: zkTLS Integration (Seeking Funding)
- [ ] TLSNotary proof generation during HTTP fetch
- [ ] DECO protocol integration (3-party TLS)
- [ ] Selective disclosure for privacy
- [ ] On-chain proof verification

### Phase 3: Production Hardening
- [ ] Security audit (smart contracts + Rust)
- [ ] Gas optimization for verifiers
- [ ] Multi-chain deployment (Solana, Stellar, Polkadot)
- [ ] Performance benchmarking on real hardware

### Phase 4: Developer Experience
- [ ] SDK for TypeScript/Python
- [ ] Web-based proof generation
- [ ] Hosted proving service
- [ ] Documentation and tutorials

---

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical design and hexagonal architecture
- **[TRUST_MODEL.md](TRUST_MODEL.md)** - Security assumptions and future roadmap
- **[DELIVERABLES.md](DELIVERABLES.md)** - Completed work and pending items
- **[USAGE.md](USAGE.md)** - Developer guide with examples

---

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific module tests
cargo test -p universal-privacy-engine-core
cargo test -p universal-privacy-engine-tee

# Run benchmarks
cargo run --release --bin benchmark
```

**Test Coverage**:
- Core: 17 tests ✅
- TEE Adapter: 10 tests ✅
- Data Ingestion: 15 tests ✅
- Verifier Generator: 5 tests ✅

---

## Contributing

This is a research prototype. Contributions welcome, but please understand:
- **Not production-ready**: Use at your own risk
- **Breaking changes**: API may change significantly
- **Grant-funded development**: Major features pending funding

---

## License

MIT OR Apache-2.0

---

## Acknowledgments

Built with:
- [SP1](https://github.com/succinctlabs/sp1) - zkVM for zero-knowledge proofs
- [Tera](https://github.com/Keats/tera) - Template engine for code generation
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for data ingestion

---

## Contact

For grant proposals, collaborations, or technical questions:
- **GitHub Issues**: [Report bugs or request features](https://github.com/your-org/universal-privacy-engine/issues)
- **Email**: [your-email@example.com]

---

**⚠️ Disclaimer**: This is research software. Do not use in production without proper security audits and hardware TEE integration.
