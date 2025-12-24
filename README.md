# Universal Privacy Engine

> A modular, production-grade Rust abstraction layer for chain-agnostic zero-knowledge proving

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## 🎯 Overview

The **Universal Privacy Engine** provides a clean, modular abstraction over zero-knowledge proving systems, enabling developers to:

- ✨ **Swap ZK backends** without changing application code (SP1, RISC0, etc.)
- 🔗 **Deploy to multiple chains** with a single codebase (Solana, Stellar, EVM)
- 🚀 **Build production-grade** privacy-preserving applications
- 🧪 **Test easily** with mockable interfaces

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Your Application                      │
│              (Chain-Agnostic Business Logic)             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
         ┌───────────────────────┐
         │   PrivacyEngine Trait │  ◄── Core Abstraction
         └───────────────────────┘
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
   ┌──────────┐          ┌──────────┐
   │ SP1      │          │ RISC0    │  ◄── Pluggable Backends
   │ Adapter  │          │ Adapter  │
   └──────────┘          └──────────┘
         │                       │
         ▼                       ▼
   ┌──────────┐          ┌──────────┐
   │ Solana   │          │ Stellar  │  ◄── Multi-Chain Export
   │ Stellar  │          │ EVM      │
   │ EVM      │          │          │
   └──────────┘          └──────────┘
```

### Design Philosophy

**Abstraction over Implementation**: By programming against the `PrivacyEngine` trait, your application remains decoupled from specific ZK backends. This enables:

1. **Backend Flexibility**: Swap from SP1 to RISC0 with a single line change
2. **Future-Proofing**: New proving systems can be integrated without refactoring
3. **Testability**: Mock implementations for unit testing without real provers
4. **Performance Optimization**: Choose the best backend for your use case

## 📦 Workspace Structure

```
universal-privacy-engine/
├── core/                   # Core abstraction layer
│   └── src/
│       └── lib.rs         # PrivacyEngine trait, ChainType, ProofReceipt
├── adapters/
│   └── sp1/               # SP1 backend implementation
│       └── src/
│           └── lib.rs     # Sp1Backend struct
└── cli/                   # Command-line interface
    └── src/
        └── main.rs        # prove, verify, export-verifier commands
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (2021 edition)
- SP1 SDK (for the SP1 adapter)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/universal-privacy-engine
cd universal-privacy-engine

# Build the workspace
cargo build --release

# Run the CLI
cargo run -p universal-privacy-engine-cli -- --help
```

### Usage Examples

#### 1. Generate a Proof

```bash
# Prepare your guest program ELF
# (This would be your compiled RISC-V ZK program)

# Generate a proof from hex-encoded input
cargo run -p universal-privacy-engine-cli -- prove \
  --input "48656c6c6f20576f726c64" \
  --elf guest.elf \
  --output proof.bin
```

#### 2. Verify a Proof

```bash
cargo run -p universal-privacy-engine-cli -- verify \
  --receipt proof.bin \
  --elf guest.elf
```

#### 3. Export a Verifier

```bash
# Export for Solana
cargo run -p universal-privacy-engine-cli -- export-verifier \
  --chain solana \
  --elf guest.elf \
  --output verifier.so

# Export for EVM
cargo run -p universal-privacy-engine-cli -- export-verifier \
  --chain evm \
  --elf guest.elf \
  --output verifier.bin
```

## 💻 Library Usage

### Using the SP1 Backend

```rust
use universal_privacy_engine_core::{PrivacyEngine, ChainType};
use universal_privacy_engine_sp1::Sp1Backend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load your guest program
    let elf = std::fs::read("guest.elf")?;
    
    // Initialize the backend
    let engine = Sp1Backend::new(elf);
    
    // Generate a proof
    let input = b"secret data";
    let receipt = engine.prove(input)?;
    
    // Verify the proof
    let is_valid = engine.verify(&receipt)?;
    assert!(is_valid);
    
    // Export a verifier for Solana
    let verifier = engine.export_verifier(ChainType::Solana)?;
    std::fs::write("verifier.so", verifier)?;
    
    Ok(())
}
```

### Swapping Backends

```rust
// Easy to swap backends without changing business logic
fn create_engine() -> Box<dyn PrivacyEngine> {
    #[cfg(feature = "sp1")]
    return Box::new(Sp1Backend::new(elf));
    
    #[cfg(feature = "risc0")]
    return Box::new(Risc0Backend::new(elf));
}
```

## 🔧 Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p universal-privacy-engine-core
```

### Adding a New Backend

1. Create a new adapter crate: `adapters/your-backend/`
2. Implement the `PrivacyEngine` trait
3. Add to workspace members in root `Cargo.toml`
4. Update CLI to support the new backend

Example:

```rust
use universal_privacy_engine_core::{PrivacyEngine, ProofReceipt, PrivacyEngineError, ChainType};

pub struct YourBackend {
    // Backend-specific fields
}

impl PrivacyEngine for YourBackend {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // Your implementation
    }
    
    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError> {
        // Your implementation
    }
    
    fn export_verifier(&self, chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError> {
        // Your implementation
    }
}
```

## 🎯 Roadmap

- [x] Core abstraction layer with `PrivacyEngine` trait
- [x] SP1 backend adapter
- [x] CLI tool for proof generation and verification
- [ ] RISC0 backend adapter
- [ ] Precompile optimizations (SHA256, ECDSA, Keccak)
- [ ] Full verifier export implementations for all chains
- [ ] Benchmarking suite
- [ ] Integration tests with live chains
- [ ] Documentation and tutorials

## 🔬 Technical Details

### Precompile Optimizations

The SP1 adapter includes placeholders for precompile syscalls that can dramatically improve proving performance:

- **SHA256**: 10-100x faster than native RISC-V implementation
- **ECDSA**: Optimized elliptic curve operations
- **Keccak**: Efficient hashing for Ethereum compatibility

These will be implemented in future versions.

### Chain-Specific Verifiers

Each blockchain has unique requirements for verifier deployment:

- **Solana**: BPF bytecode with account-based state management
- **Stellar**: WASM module compatible with Soroban runtime
- **EVM**: Solidity contract with optimized gas usage

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📚 Resources

- [SP1 Documentation](https://docs.succinct.xyz/)
- [RISC0 Documentation](https://dev.risczero.com/)
- [Zero-Knowledge Proofs Explained](https://z.cash/technology/zksnarks/)

---

**Built with ❤️ for the Deep Tech community**
