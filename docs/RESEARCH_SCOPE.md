# Research Scope — Oasis Sapphire Institutional Privacy Layer

## Purpose of This Document

This document defines the **research scope** for the Universal Privacy Engine (UPE) as part of the **Oasis ROSE Bloom Grant** application. UPE is positioned as **privacy-preserving infrastructure** built exclusively for Oasis Sapphire's Confidential EVM.

---

## Research Objective

### Primary Question

**How can institutions settle sensitive off-chain data into verifiable on-chain state without exposing confidential information?**

### Hypothesis

By combining:
1. **Oasis Sapphire's encrypted state** (confidentiality guarantee)
2. **STLOP cryptographic proofs** (authenticity guarantee)
3. **Smart contract logic** (programmable compliance)

We can create an **Institutional Privacy Layer** that enables regulated industries (finance, healthcare, HR) to leverage blockchain infrastructure while maintaining data privacy.

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

## Research Scope: What's In Scope

### 1. STLOP Proof System

**Objective**: Design and implement a lightweight proof system for off-chain data ingestion.

**Components**:
- **Rust Notary Service**: Fetches data from external APIs, signs with Ed25519/ECDSA
- **On-Chain Verifier**: Solidity contract validates signatures (EIP-191 format)
- **Proof Format**: `(data, timestamp, signature)` tuple

**Research Questions**:
- What is the optimal signature scheme for gas efficiency on Sapphire?
- How do we prevent replay attacks across different contracts?
- Can we batch multiple proofs into a single transaction?

### 2. Institutional Use Cases

**Primary Use Case**: Private Payroll Settlement

**Scenario**: A company wants to:
- Prove employee salaries on-chain (for loan applications, financial services)
- Maintain confidentiality (GDPR, SOC2 compliance)
- Enable employee self-verification (no HR intermediary)

**Implementation**: `PrivatePayroll.sol` contract on Sapphire

**Research Questions**:
- What access control patterns work best for confidential state?
- How do we handle salary updates (immutability vs. mutability)?
- Can we support range proofs ("salary > $50k") without revealing exact amounts?

**Secondary Use Cases** (Future Work):
- **Compliance Records**: KYC/AML data for DeFi protocols
- **Financial Statements**: Private balance sheets for institutional DeFi
- **Healthcare Records**: HIPAA-compliant medical data on-chain

### 3. Sapphire-Specific Optimizations

**Objective**: Leverage Sapphire's unique features beyond basic encrypted state.

**Research Areas**:
- **Confidential Randomness**: Using Sapphire's VRF for fair salary audits
- **Encrypted Events**: Emitting logs that only authorized parties can decrypt
- **Cross-Contract Calls**: Maintaining confidentiality across contract boundaries

**Research Questions**:
- How does gas cost compare to standard EVM chains?
- What are the performance limits of encrypted state queries?
- Can we use Sapphire's precompiles for cryptographic operations?

### 4. ROFL Integration Roadmap

**Objective**: Plan future integration with Runtime Off-chain Logic (ROFL).

**ROFL Benefits**:
- **Decentralized Notary**: Replace single trusted signer with MPC cluster
- **Off-Chain Computation**: Complex data processing before settlement
- **Enhanced Privacy**: Combine zkTLS with Sapphire's encrypted state

**Research Questions**:
- What is the optimal architecture for ROFL + Sapphire integration?
- How do we handle ROFL attestation verification on-chain?
- Can we use ROFL for zkTLS proof generation?

---

## Research Scope: What's Out of Scope

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

**Future Work**: ROFL-based zkTLS is planned for Phase 3 (requires 6-9 months additional development).

### ❌ Universal Protocol Claims

**Not in Scope**: Positioning UPE as a "universal" or "cross-chain" protocol.

**Rationale**: This dilutes the Oasis-specific narrative. UPE is **Sapphire-native infrastructure**, not a multi-chain abstraction layer.

---

## Research Methodology

### Phase 1: Proof of Concept (✅ Complete)

**Objective**: Validate that STLOP proofs can be verified on Sapphire.

**Deliverables**:
- [x] `PrivatePayroll.sol` contract with signature verification
- [x] Rust notary service for proof generation
- [x] Testnet deployment and manual testing

**Outcome**: ✅ Successfully demonstrated encrypted state storage with STLOP proofs.

### Phase 2: Documentation & Demo (🚧 In Progress)

**Objective**: Create grant-ready materials for Oasis reviewers.

**Deliverables**:
- [x] Oasis-focused README.md
- [x] Architecture documentation
- [x] Trust model analysis
- [ ] Demo video (ETA: 1 week)
- [ ] Architecture diagrams (ETA: 3 days)

**Outcome**: 90% complete, final deliverables due January 16, 2026.

### Phase 3: ROFL Roadmap (📋 Planned)

**Objective**: Design future integration with Oasis ROFL.

**Deliverables**:
- [ ] ROFL architecture specification
- [ ] MPC signing design
- [ ] zkTLS integration plan
- [ ] Sapphire contract updates for ROFL attestation

**Timeline**: 6-9 months (requires additional funding).

---

## Success Metrics

### Technical Metrics

- ✅ **Contract Deployment**: `PrivatePayroll.sol` verified on Sapphire Testnet
- ✅ **Proof Verification**: STLOP signatures validated on-chain
- ✅ **Encrypted State**: `getMySalary()` returns correct data only to employee
- ✅ **Gas Efficiency**: ~50,000 gas per proof verification (competitive)

### Research Metrics

- ✅ **Novel Proof System**: STLOP methodology documented and implemented
- ✅ **Institutional Use Case**: Private payroll demonstrated
- ✅ **Sapphire Integration**: Leverages encrypted state, not just generic EVM
- 🚧 **Documentation**: Grant-ready materials (90% complete)

### Ecosystem Metrics

- 📋 **Developer Adoption**: SDK and tutorials (future work)
- 📋 **Institutional Pilots**: Partnerships with payroll providers (future work)
- 📋 **Community Engagement**: Oasis Discord, developer workshops (future work)

---

## Institutional Privacy Layer Vision

### Target Industries

1. **Human Resources**: Private payroll, benefits verification, employment history
2. **Financial Services**: Confidential balance sheets, credit scores, loan applications
3. **Healthcare**: HIPAA-compliant medical records, insurance claims
4. **Compliance**: KYC/AML data for DeFi, regulatory reporting

### Why Institutions Need This

**Current Problem**: Institutions cannot use public blockchains because:
- Regulatory requirements (GDPR, HIPAA, SOC2) prohibit public data exposure
- Competitive concerns (financial data, trade secrets)
- Customer privacy expectations

**UPE + Sapphire Solution**:
- ✅ **Regulatory Compliance**: Encrypted state meets privacy requirements
- ✅ **Verifiability**: Cryptographic proofs enable audits without exposure
- ✅ **Decentralization**: Public blockchain benefits without privacy trade-offs

---

## Comparison to Alternatives

### vs. Zero-Knowledge Proofs (zkSNARKs)

| Feature | zkSNARKs | UPE + Sapphire |
|---------|----------|----------------|
| **Privacy** | ✅ Selective disclosure | ✅ Full encrypted state |
| **Complexity** | ❌ High (circuit design) | ✅ Low (standard Solidity) |
| **Gas Cost** | ❌ Expensive (~500k gas) | ✅ Efficient (~50k gas) |
| **State Storage** | ⚠️ Limited | ✅ Unlimited |
| **Developer UX** | ❌ Steep learning curve | ✅ Familiar EVM tooling |

**Conclusion**: UPE + Sapphire is more practical for institutional use cases requiring large amounts of confidential state.

### vs. Private Blockchains (Hyperledger)

| Feature | Hyperledger | UPE + Sapphire |
|---------|-------------|----------------|
| **Privacy** | ✅ Full | ✅ Full |
| **Decentralization** | ❌ Permissioned | ✅ Public blockchain |
| **Censorship Resistance** | ❌ Low | ✅ High |
| **Interoperability** | ❌ Siloed | ✅ Oasis ecosystem |
| **Composability** | ❌ Limited | ✅ DeFi integration |

**Conclusion**: UPE + Sapphire provides privacy **without sacrificing** decentralization.

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

**Last Updated**: January 2, 2026  
**Grant Program**: Oasis ROSE Bloom  
**Research Status**: Phase 2 (Documentation & Demo)
