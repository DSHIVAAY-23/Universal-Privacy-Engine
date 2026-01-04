# Research Scope — Privacy-Preserving Data Ingestion (Oasis Sapphire)

## Objective

Focused research areas that directly support Sapphire adoption:
- Practical STLOP design tradeoffs for signed TLS observation proofs.
- Migration path from a single trusted notary (Phase 1) to a ROFL-based decentralized notary network (Phase 2).
- Minimal on-chain verifier complexity to reduce gas & attack surface on Sapphire.

## Phase 1 — Trusted Notary

**Research Questions**:
- What is the optimal signature scheme for gas efficiency on Sapphire?
- How do we prevent replay attacks across different contracts?
- Can we batch multiple proofs into a single transaction?
- What access control patterns work best for confidential state?

**Deliverables**:
- Rust notary service with Ed25519/ECDSA signing
- `PrivatePayroll.sol` contract with STLOP verification
- Documentation of trust assumptions and security tradeoffs

**Status**: Complete

## Phase 2 — ROFL Integration

**Research Questions**:
- What is the optimal architecture for ROFL + Sapphire integration?
- How do we handle ROFL attestation verification on-chain?
- Can we use ROFL for zkTLS proof generation?
- What are the gas cost implications of MPC signature verification?

**Deliverables** (Planned):
- ROFL architecture specification
- MPC signing design for decentralized notary cluster
- zkTLS integration plan (TLSNotary or similar)
- Sapphire contract updates for ROFL attestation verification

**Timeline**: 6-9 months (requires additional funding)

**Status**: Roadmap defined, implementation pending

---

## Why Oasis Sapphire?

### The Institutional Privacy Problem

Traditional blockchains force a binary choice:

| Approach | Transparency | Privacy | Institutional Viability |
|----------|--------------|---------|-------------------------|
| **Public Blockchain** (Ethereum, Polygon) | ✅ Full | ❌ None | ❌ Unacceptable for sensitive data |
| **Private Blockchain** (Hyperledger, Corda) | ❌ Limited | ✅ Full | ⚠️ Loses decentralization benefits |
| **Zero-Knowledge Proofs** (zkSNARKs) | ✅ Verifiable | ✅ Selective | ⚠️ Complex, expensive, limited state |

**Oasis Sapphire's Unique Solution**:
- ✅ **Public blockchain** (decentralization, censorship resistance)
- ✅ **Encrypted state** (confidentiality by default)
- ✅ **EVM compatibility** (existing tooling, developer familiarity)

### Sapphire's Confidential EVM Advantage

```solidity
// On Ethereum/Polygon/BSC:
mapping(address => uint256) private salaries; 
// ❌ "private" keyword is a LIE - anyone can read this from storage

// On Oasis Sapphire:
mapping(address => uint256) private salaries;
// ✅ ACTUALLY PRIVATE - encrypted at ParaTime level
```

**Technical Mechanism**: Sapphire's ParaTime uses **Trusted Execution Environments (TEEs)** to encrypt all contract state. Even validators cannot read the plaintext data.

---

## Institutional Use Cases

### Primary Use Case: Private Payroll Settlement

**Scenario**: A company wants to:
- Prove employee salaries on-chain (for loan applications, financial services)
- Maintain confidentiality (GDPR, SOC2 compliance)
- Enable employee self-verification (no HR intermediary)

**Implementation**: `PrivatePayroll.sol` contract on Sapphire

**Research Questions**:
- How do we handle salary updates (immutability vs. mutability)?
- Can we support range proofs ("salary > $50k") without revealing exact amounts?
- What are the gas costs for batch salary updates?

### Secondary Use Cases (Future Work)

- **Compliance Records**: KYC/AML data for DeFi protocols
- **Financial Statements**: Private balance sheets for institutional DeFi
- **Healthcare Records**: HIPAA-compliant medical data on-chain
- **Identity Verification**: Privacy-preserving credential verification

---

## Research Methodology

### Success Metrics

#### Technical Metrics

- ✅ **Contract Deployment**: `PrivatePayroll.sol` verified on Sapphire Testnet
- ✅ **Proof Verification**: STLOP signatures validated on-chain
- ✅ **Encrypted State**: `getMySalary()` returns correct data only to employee
- ✅ **Gas Efficiency**: ~50,000 gas per proof verification (competitive)

#### Research Metrics

- ✅ **Novel Proof System**: STLOP methodology documented and implemented
- ✅ **Institutional Use Case**: Private payroll demonstrated
- ✅ **Sapphire Integration**: Leverages encrypted state, not just generic EVM
- 🚧 **Documentation**: Grant-ready materials (90% complete)

#### Ecosystem Metrics (Future)

- 📋 **Developer Adoption**: SDK and tutorials
- 📋 **Institutional Pilots**: Partnerships with payroll providers
- 📋 **Community Engagement**: Oasis Discord, developer workshops

---

## What's In Scope

### ✅ Oasis-Focused Research

- STLOP proof system design and implementation
- Sapphire-specific optimizations (encrypted state, confidential randomness)
- ROFL integration roadmap
- Institutional privacy use cases (payroll, compliance, finance)

### ✅ Grant Deliverables

- `PrivatePayroll.sol` contract suite
- Rust notary service
- Comprehensive documentation
- Demo video and walkthrough
- Sapphire testnet deployment

---

## What's Out of Scope

### ❌ Multi-Chain Deployments

**Not in Scope**: Deploying UPE to Ethereum, Polygon, Solana, Stellar, or other chains.

**Rationale**: This grant is **Oasis-exclusive**. Sapphire's encrypted state is the core innovation. Other chains cannot provide the same confidentiality guarantees.

**Note**: The repository may contain legacy code for other chains (from prior research), but these are **not part of the grant deliverables**.

### ❌ Production-Ready System

**Not in Scope**: Formal security audits, mainnet deployment, production SLAs.

**Rationale**: This is **research infrastructure** to demonstrate feasibility. Production hardening requires additional funding and timeline.

**Current Status**: Alpha prototype on Sapphire Testnet.

### ❌ zkTLS Integration (Current Phase)

**Not in Scope**: Full TLSNotary or DECO protocol integration.

**Rationale**: zkTLS is a future enhancement (ROFL roadmap). The current grant focuses on STLOP proofs with a trusted notary.

**Future Work**: ROFL-based zkTLS is planned for Phase 2 (requires 6-9 months additional development).

---

## Honest Assessment

### What We've Proven

✅ **Technical Feasibility**: STLOP proofs work on Sapphire  
✅ **Encrypted State**: Confidential data storage is functional  
✅ **Developer Experience**: Standard Solidity contracts (no custom tooling)  
✅ **Institutional Relevance**: Private payroll is a real-world use case  

### What We Haven't Proven (Yet)

⚠️ **Scalability**: Limited testing with large datasets  
⚠️ **Decentralization**: Single trusted notary (ROFL will address)  
⚠️ **Production Readiness**: No formal security audit  
⚠️ **Economic Viability**: No cost analysis for institutional deployment  

### What's Beyond This Grant

🔮 **zkTLS Integration**: Cryptographic TLS proofs (6-9 months)  
🔮 **ROFL Notary**: Decentralized MPC signing (6-9 months)  
🔮 **Multi-Use Cases**: Healthcare, finance, compliance (12+ months)  
🔮 **Mainnet Deployment**: Production hardening and audit (12+ months)  

---

## Conclusion

The Universal Privacy Engine is **Oasis-native research infrastructure** designed to unlock institutional blockchain adoption through Sapphire's Confidential EVM.

**This grant focuses on**:
- ✅ Demonstrating STLOP proof methodology
- ✅ Building PrivatePayroll reference implementation
- ✅ Creating grant-ready documentation
- 📋 Planning ROFL integration roadmap

**This grant does NOT include**:
- ❌ Multi-chain deployments
- ❌ Production-ready systems
- ❌ zkTLS integration (future work)
- ❌ Formal security audits

**Research Contribution**: UPE demonstrates that Sapphire's encrypted state + lightweight cryptographic proofs can solve the institutional privacy problem **without** the complexity of zkSNARKs or the centralization of private blockchains.

---

**Last Updated**: January 4, 2026  
**Grant Program**: Oasis ROSE Bloom  
**Research Status**: Phase 1 Complete, Phase 2 Roadmap Defined
