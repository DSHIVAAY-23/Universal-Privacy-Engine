# Architecture

## Overview

The Universal Privacy Engine follows a **Hexagonal Architecture** (Ports and Adapters pattern) to achieve backend independence and modularity. This design allows swapping proving systems (SP1, RISC0, TEEs) without changing application logic.

---

## Hexagonal Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                            │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │     CLI      │  │     API      │  │ Data Ingest  │             │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘             │
│         │                  │                  │                      │
│         └──────────────────┼──────────────────┘                      │
│                            │                                         │
└────────────────────────────┼─────────────────────────────────────────┘
                             │
                             ▼
              ┌──────────────────────────────┐
              │      PORT (Interface)        │
              │                              │
              │    ┌──────────────────┐     │
              │    │ PrivacyEngine    │     │
              │    │     Trait        │     │
              │    │                  │     │
              │    │ • prove()        │     │
              │    │ • verify()       │     │
              │    │ • export_veri... │     │
              │    └──────────────────┘     │
              │                              │
              └──────────────┬───────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
┌────────────────┐  ┌────────────────┐  ┌────────────────┐
│   ADAPTER 1    │  │   ADAPTER 2    │  │   ADAPTER 3    │
│                │  │                │  │                │
│  Sp1Backend    │  │ TeeProverStub  │  │ Future: RISC0  │
│                │  │                │  │                │
│  • SP1 SDK     │  │ • Mock TEE     │  │ • RISC0 SDK    │
│  • Groth16     │  │ • Ed25519      │  │ • STARK        │
│  • PLONK       │  │ • Attestation  │  │                │
└────────────────┘  └────────────────┘  └────────────────┘
     ZK-VM              TEE Mock            ZK-VM
```

---

## Core Abstraction: `PrivacyEngine` Trait

The `PrivacyEngine` trait is the **Port** in our hexagonal architecture. All proving backends must implement this interface.

### Trait Definition

```rust
pub trait PrivacyEngine {
    /// Generate a zero-knowledge proof or TEE attestation
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError>;
    
    /// Verify a proof or attestation
    fn verify(&self, receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError>;
    
    /// Export verifier contract for blockchain deployment
    fn export_verifier(&self, chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError>;
}
```

### Key Design Decisions

1. **Blocking I/O**: Methods are synchronous (CPU-bound operations)
2. **Opaque Input**: `&[u8]` allows any serialization format
3. **Unified Receipt**: `ProofReceipt` works for both ZK proofs and TEE attestations
4. **Chain-Agnostic**: `export_verifier()` supports multiple blockchains

---

## Proof Type Abstraction

The `ProofType` enum distinguishes between fundamentally different cryptographic primitives:

```rust
pub enum ProofType {
    /// Zero-knowledge proof from a zkVM (e.g., SP1, RISC0)
    ZkProof,
    
    /// Attestation from a Trusted Execution Environment (e.g., Intel SGX, AWS Nitro)
    TeeAttestation,
}
```

### Comparison

| Aspect | ZK Proof | TEE Attestation |
|--------|----------|-----------------|
| **Proof Size** | 300 bytes (Groth16) - 10MB (STARK) | 1-5KB |
| **Generation Time** | 30s - 3min | ~200ms |
| **Trust Model** | Cryptographic (no trust) | Hardware vendor trust |
| **Verification** | On-chain or off-chain | Typically off-chain |
| **Privacy** | Zero-knowledge | Trusted execution |

---

## Data Ingestion Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                    Data Ingestion Layer                      │
│                                                              │
│  ┌────────────────┐                                         │
│  │ DataProvider   │  ◄── Port (Trait)                       │
│  │     Trait      │                                         │
│  └────────┬───────┘                                         │
│           │                                                  │
│           ▼                                                  │
│  ┌────────────────┐         ┌──────────────────┐           │
│  │ HttpProvider   │────────▶│  reqwest Client  │           │
│  │                │         └──────────────────┘           │
│  │ • fetch()      │                                         │
│  │ • parse_json() │         ┌──────────────────┐           │
│  │ • verify_tls() │────────▶│ zkTLS Stub (TODO)│           │
│  └────────┬───────┘         └──────────────────┘           │
│           │                                                  │
│           ▼                                                  │
│  ┌────────────────┐                                         │
│  │ZkInputBuilder  │                                         │
│  │                │                                         │
│  │ Data + Secrets │────────▶ PrivacyEngine Input           │
│  └────────────────┘                                         │
└─────────────────────────────────────────────────────────────┘
```

### Components

1. **`DataProvider` Trait**: Async interface for fetching external data
2. **`HttpProvider`**: HTTP implementation with JSON path selector
3. **`ZkInputBuilder`**: Combines public data with secrets securely
4. **zkTLS Stub**: Placeholder for future TLSNotary/DECO integration

---

## Adapter Implementations

### SP1 Adapter (`adapters/sp1`)

**Purpose**: Zero-knowledge proof generation using SP1 zkVM

**Key Features**:
- Compiles Rust guest programs to RISC-V
- Generates Groth16/PLONK proofs
- Exports Solidity verifiers
- Supports RWA compliance use cases

**Proof Flow**:
```
Rust Guest Program → SP1 Compiler → RISC-V ELF
                                         ↓
                                    SP1 Prover
                                         ↓
                                  Groth16 Proof
                                         ↓
                                  ProofReceipt
```

### TEE Adapter (`adapters/tee`)

**Purpose**: Mock TEE attestations for development

**Current State**: Simulation only (no real SGX/Nitro)

**Mock Implementation**:
- Generates Ed25519 signatures
- Simulates 200ms computation delay
- Returns `MockAttestation` struct
- **⚠️ NOT cryptographically secure**

**Future Implementation**:
```
User Input → SGX Enclave → Secure Computation
                               ↓
                          DCAP Quote
                               ↓
                        TEE Attestation
                               ↓
                        ProofReceipt
```

---

## Smart Contract Generation

```
┌─────────────────────────────────────────────────────────────┐
│              Verifier Generation Pipeline                    │
│                                                              │
│  ┌──────────────┐                                           │
│  │ CLI Script   │                                           │
│  │ (generate_   │                                           │
│  │  verifier)   │                                           │
│  └──────┬───────┘                                           │
│         │                                                    │
│         │ --program-vkey <HASH>                             │
│         │ --out-dir <PATH>                                  │
│         ▼                                                    │
│  ┌──────────────┐         ┌──────────────────┐             │
│  │ Tera Engine  │────────▶│ Template File    │             │
│  │              │         │ (UniversalVeri-  │             │
│  │              │         │  fier.sol.tera)  │             │
│  └──────┬───────┘         └──────────────────┘             │
│         │                                                    │
│         │ Render with VKey                                  │
│         ▼                                                    │
│  ┌──────────────┐                                           │
│  │ Generated    │                                           │
│  │ Solidity     │                                           │
│  │ Contract     │                                           │
│  └──────────────┘                                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Module Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                         Workspace                            │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                    core                               │  │
│  │  • PrivacyEngine trait                               │  │
│  │  • ProofReceipt, ProofType                           │  │
│  │  • DataProvider trait                                │  │
│  │  • Error types                                       │  │
│  └──────────────────┬───────────────────────────────────┘  │
│                     │                                       │
│         ┌───────────┼───────────┐                          │
│         │           │           │                          │
│         ▼           ▼           ▼                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐                   │
│  │   sp1    │ │   tee    │ │   bin    │                   │
│  │ adapter  │ │ adapter  │ │benchmark │                   │
│  └──────────┘ └──────────┘ └──────────┘                   │
│                                                              │
│  ┌──────────┐                                               │
│  │ scripts  │                                               │
│  │ generate │                                               │
│  │ verifier │                                               │
│  └──────────┘                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Future Architecture Evolution

### Phase 1: Real TEE Integration

Replace `TeeProverStub` with actual hardware adapters:

```rust
// Intel SGX Adapter
pub struct SgxProver {
    enclave: SgxEnclave,
    attestation_config: DcapConfig,
}

impl PrivacyEngine for SgxProver {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // 1. Copy input to enclave memory
        // 2. Execute computation in enclave
        // 3. Generate DCAP quote
        // 4. Return attestation
    }
}

// AWS Nitro Adapter
pub struct NitroProver {
    enclave_config: NitroConfig,
}

impl PrivacyEngine for NitroProver {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // 1. Generate attestation document
        // 2. Sign with AWS KMS
        // 3. Return proof receipt
    }
}
```

### Phase 2: zkTLS Integration

Enhance `HttpProvider` with cryptographic data authenticity:

```rust
impl HttpProvider {
    async fn fetch(&self, source: &str, query: &str) -> Result<Vec<u8>, DataError> {
        // 1. Capture TLS session
        let (proof, response) = tlsn_prover.prove_tls_session(source).await?;
        
        // 2. Store proof
        self.tls_proof = Some(proof);
        
        // 3. Extract field
        self.select_json_field(&response, query)
    }
    
    fn verify_tls_signature(&self) -> bool {
        // 1. Verify notary signature
        // 2. Check timestamp freshness
        // 3. Validate certificate chain
        verifier.verify(self.tls_proof.as_ref()?).is_ok()
    }
}
```

### Phase 3: Multi-Chain Support

Extend `export_verifier()` for multiple blockchains:

```rust
match chain {
    ChainType::Ethereum => generate_evm_verifier(...),
    ChainType::Solana => generate_solana_verifier(...),
    ChainType::Stellar => generate_stellar_verifier(...),
    ChainType::Polkadot => generate_wasm_verifier(...),
}
```

---

## Design Principles

1. **Separation of Concerns**: Core abstractions independent of implementations
2. **Dependency Inversion**: High-level modules don't depend on low-level details
3. **Open/Closed Principle**: Open for extension (new adapters), closed for modification
4. **Interface Segregation**: Minimal trait interfaces
5. **Single Responsibility**: Each module has one reason to change

---

## Performance Considerations

### ZK Proof Generation (SP1)
- **Mock Mode**: 100ms - 5s (for development)
- **STARK Mode**: 30s - 60s (production)
- **Groth16 Mode**: 2min - 3min (on-chain verification)

### TEE Attestation (Future)
- **SGX DCAP**: ~200ms - 500ms
- **AWS Nitro**: ~100ms - 300ms
- **Azure CC**: ~150ms - 400ms

### Data Ingestion
- **HTTP Fetch**: Network latency dependent
- **JSON Parsing**: <10ms for typical responses
- **zkTLS Proof**: +500ms overhead (future)

---

## Security Architecture

### Current (Alpha)
- ⚠️ Client-side proving (no isolation)
- ⚠️ Mock TEE (no hardware security)
- ⚠️ No zkTLS (trust HTTPS endpoints)

### Target (Production)
- ✅ Hardware-backed TEEs (SGX/Nitro)
- ✅ Cryptographic data authenticity (zkTLS)
- ✅ Audited smart contracts
- ✅ Multi-party computation options

---

## References

- **Hexagonal Architecture**: [Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- **SP1 zkVM**: [Succinct Labs](https://github.com/succinctlabs/sp1)
- **Intel SGX**: [Intel SGX Documentation](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/overview.html)
- **AWS Nitro**: [AWS Nitro Enclaves](https://aws.amazon.com/ec2/nitro/nitro-enclaves/)
- **TLSNotary**: [TLSNotary Protocol](https://tlsnotary.org/)
