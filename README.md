# Universal Privacy Engine (UPE) — Oasis Sapphire Institutional Privacy Layer

> **Status: Research Prototype for Oasis ROSE Bloom Grant**  
> Middleware infrastructure that transforms off-chain data into **privacy-preserving smart contract state** on Oasis Sapphire's Confidential EVM.

---

## Overview

The **Universal Privacy Engine (UPE)** is a privacy-preserving middleware framework built specifically for **Oasis Sapphire**, the first and only Confidential EVM. UPE enables institutions to settle sensitive off-chain data (payroll, compliance records, financial statements) into **encrypted on-chain state** without exposing plaintext information.

### Why Oasis Sapphire?

Traditional smart contracts store all state publicly on-chain. Even "private" mappings in Solidity are readable by anyone with an archive node. **Oasis Sapphire changes this** by providing:

- **Encrypted State by Default**: All contract storage is encrypted at the ParaTime level
- **Confidential Computation**: Smart contracts can process sensitive data without exposure
- **EVM Compatibility**: Deploy standard Solidity contracts with automatic privacy guarantees

UPE leverages these unique capabilities to create an **Institutional Privacy Layer** for regulated industries.

---

## Key Features

### 🔐 **STLOP: Signed TLS Off-chain Proofs**

UPE introduces a novel proof system for ingesting off-chain data:

1. **Rust Notary Service** fetches data from external APIs (payroll systems, banks, compliance databases)
2. **Cryptographic Signing** creates tamper-proof attestations of the data
3. **On-Chain Verification** validates signatures before storing data in Sapphire's encrypted state

**Example Use Case**: A company's payroll system generates salary data. The UPE Notary signs this data, and employees can verify their salary on-chain **without revealing it publicly**.

### 🏗️ **PrivatePayroll Contract Suite**

Our flagship demonstration contract (`contracts/oasis/src/PrivatePayroll.sol`) showcases:

- **Confidential Storage**: Salary data stored in Sapphire's encrypted state
- **Proof Verification**: EIP-191 signature validation from trusted notary
- **Access Control**: Only employees can view their own salary via `getMySalary()`

```solidity
// On a normal EVM chain, this mapping is PUBLIC
// On Sapphire, it is PRIVATE by default
mapping(address => uint256) private salaries;
```

### 🚀 **ROFL Integration Roadmap**

Future work will integrate **ROFL (Runtime Off-chain Logic)** for:

- **Decentralized Notary**: Replace single trusted notary with ROFL-based MPC
- **Off-Chain Computation**: Complex data processing before on-chain settlement
- **Enhanced Privacy**: Zero-knowledge proofs combined with confidential state

---

## Quick Demo

### Prerequisites

```bash
# Install dependencies
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox

# Set up Oasis Sapphire Testnet in hardhat.config.js
```

### Step 1: Compile Contracts

```bash
cd contracts/oasis
npx hardhat compile
```

### Step 2: Deploy to Sapphire Testnet

```bash
npx hardhat run scripts/deploy.js --network sapphire-testnet
```

### Step 3: Generate STLOP Proof

```bash
# Run the Rust notary to sign salary data
cargo run --bin notary -- \
  --employee 0xYourAddress \
  --salary 75000 \
  --timestamp $(date +%s)
```

### Step 4: Verify and Store Salary

```bash
# Submit the signed proof to PrivatePayroll contract
npx hardhat run scripts/verify_salary.js --network sapphire-testnet
```

### Step 5: Query Encrypted State

```bash
# Only the employee can see their own salary
npx hardhat run scripts/get_salary.js --network sapphire-testnet
# Output: 75000 (encrypted on-chain, visible only to you)
```

**🎥 Demo Video**: [Coming Soon - Grant Deliverable]

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Off-Chain Layer                          │
│                                                             │
│  ┌──────────────┐         ┌─────────────────┐             │
│  │ Payroll API  │────────▶│  Rust Notary    │             │
│  │ (External)   │         │  (STLOP Signer) │             │
│  └──────────────┘         └────────┬────────┘             │
│                                     │                       │
└─────────────────────────────────────┼───────────────────────┘
                                      │
                                      │ Signed Proof
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│              Oasis Sapphire ParaTime (On-Chain)             │
│                                                             │
│  ┌────────────────────────────────────────────────────┐   │
│  │         PrivatePayroll.sol Contract                │   │
│  │                                                    │   │
│  │  • verifyAndStoreSalary(salary, timestamp, sig)   │   │
│  │  • Validates notary signature (EIP-191)           │   │
│  │  • Stores in ENCRYPTED state                      │   │
│  │  • getMySalary() - Employee-only access           │   │
│  └────────────────────────────────────────────────────┘   │
│                                                             │
│         🔒 All state encrypted by Sapphire ParaTime         │
└─────────────────────────────────────────────────────────────┘
                                      │
                                      │ Future: ROFL
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│                  ROFL (Future Enhancement)                  │
│                                                             │
│  • Decentralized notary via off-chain computation          │
│  • MPC-based signing (no single point of trust)            │
│  • Complex data transformations before settlement          │
└─────────────────────────────────────────────────────────────┘
```

---

## Grant Deliverables (Oasis ROSE Bloom)

This repository is being developed as part of the **Oasis ROSE Bloom Grant** program. Our deliverables include:

### ✅ Phase 1: PrivatePayroll Contract Suite
- [x] Solidity contracts leveraging Sapphire's encrypted state
- [x] STLOP proof verification (EIP-191 signatures)
- [x] Access control for confidential data queries
- [x] Comprehensive NatSpec documentation

### 🚧 Phase 2: Documentation & Demo (In Progress)
- [ ] Architecture diagrams (Mermaid + visual assets)
- [ ] Video walkthrough of PrivatePayroll deployment
- [ ] Written tutorial for institutional use cases
- [ ] Grant notes and technical deep dive

### 🔮 Phase 3: ROFL Integration (Future)
- [ ] Decentralized notary using ROFL
- [ ] Off-chain computation for complex data processing
- [ ] Enhanced privacy with zkTLS integration

See [DELIVERABLES.md](DELIVERABLES.md) for detailed tracking.

---

## Project Structure

```
universal-privacy-engine/
├── contracts/oasis/           # Sapphire-specific contracts
│   └── src/
│       └── PrivatePayroll.sol # Flagship demo contract
├── core/                      # Rust notary service
│   └── src/
│       └── notary/            # STLOP signing logic
├── docs/                      # Grant documentation
│   └── oasis_grant_notes.md   # Demo steps & narrative
├── scripts/                   # Deployment & testing scripts
└── README.md                  # This file
```

---

## Security & Trust Model

### Current Trust Assumptions

**Alpha Prototype Status**: This is research infrastructure, not production-ready software.

- **Notary Trust**: Single trusted notary (hardcoded address in contract)
- **Data Authenticity**: Relies on notary's integrity to sign accurate data
- **Confidentiality**: Provided by Sapphire's encrypted state (strong guarantee)
- **Integrity**: Cryptographic signatures prevent tampering

### Hardening Roadmap

1. **Multi-Notary Quorum** (Phase 2): Require M-of-N signatures
2. **ROFL Integration** (Phase 3): Decentralized off-chain computation
3. **zkTLS** (Phase 4): Cryptographic proof of TLS data without trusted notary
4. **On-Chain Key Rotation** (Phase 5): Smart contract-based notary key management

See [TRUST_MODEL.md](TRUST_MODEL.md) for detailed analysis.

---

## Why Oasis Sapphire?

### The Institutional Privacy Problem

Regulated industries (finance, healthcare, HR) need to:
- **Prove compliance** without revealing sensitive data
- **Settle transactions** on-chain for auditability
- **Maintain confidentiality** to protect customer privacy

Traditional blockchains force a choice: **transparency OR privacy**. Sapphire provides **both**.

### UPE's Solution

By combining:
- **Sapphire's encrypted state** (confidentiality)
- **STLOP cryptographic proofs** (authenticity)
- **Smart contract logic** (programmable compliance)

UPE creates an **Institutional Privacy Layer** where:
- Payroll data is verifiable but not public
- Compliance records are auditable but not exposed
- Financial statements are provable but not readable

---

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)**: Technical design and Sapphire integration
- **[DELIVERABLES.md](DELIVERABLES.md)**: Grant milestones and progress tracking
- **[TRUST_MODEL.md](TRUST_MODEL.md)**: Security assumptions and hardening roadmap
- **[RESEARCH_SCOPE.md](RESEARCH_SCOPE.md)**: Oasis-focused research objectives
- **[docs/oasis_grant_notes.md](docs/oasis_grant_notes.md)**: Demo steps and grant narrative

---

## Links & Resources

- **Oasis Network**: [https://oasisprotocol.org](https://oasisprotocol.org)
- **Sapphire Documentation**: [https://docs.oasis.io/dapp/sapphire/](https://docs.oasis.io/dapp/sapphire/)
- **ROFL Documentation**: [https://docs.oasis.io/dapp/rofl/](https://docs.oasis.io/dapp/rofl/)
- **ROSE Bloom Grants**: [https://oasisprotocol.org/grants](https://oasisprotocol.org/grants)

---

## Disclaimer

**Research Prototype**: This software is for experimental and development use only. It demonstrates the feasibility of privacy-preserving institutional data settlement on Oasis Sapphire but is **not production-ready**.

**Alpha Limitations**:
- Single trusted notary (centralization risk)
- No formal security audit
- Limited testing on Sapphire Testnet
- ROFL integration not yet implemented

**Do not use with real sensitive data or production value.**

---

## License

MIT License - See [LICENSE](LICENSE) for details.

---

## Contact

For grant-related inquiries or technical questions:
- **GitHub Issues**: [Universal Privacy Engine](https://github.com/your-org/universal-privacy-engine/issues)
- **Oasis Discord**: [#sapphire-developers](https://oasis.io/discord)

---

**Built for Oasis Sapphire** 🌸 | **Powered by Confidential EVM** 🔐
