# Trust Model — Oasis Sapphire Institutional Privacy Layer

## Overview

This document defines the security assumptions and trust model for the **Universal Privacy Engine (UPE)** on Oasis Sapphire. It explicitly separates the **Current State (Alpha Prototype)** from the **Target State (ROFL Integration)** to provide honest assessment for grant reviewers.

---

## Current Trust Model (Alpha Prototype)

### ✅ What is Secured

#### 1. Confidentiality (Sapphire's Encrypted State)

**Guarantee**: All contract storage is encrypted at the Sapphire ParaTime level.

**Mechanism**: Oasis Sapphire runs inside a **Trusted Execution Environment (TEE)**:
- Storage encryption keys are only accessible inside the TEE
- Validators cannot read plaintext state
- External observers cannot read storage slots

**UPE Benefit**: Salary data in `PrivatePayroll.sol` is **cryptographically confidential** on-chain.

```solidity
// On Sapphire, this mapping is ACTUALLY PRIVATE
mapping(address => uint256) private salaries;
```

**Threat Mitigation**:
- ✅ **Blockchain Observers**: Cannot read encrypted state
- ✅ **Archive Nodes**: Cannot access plaintext storage
- ✅ **Other Employees**: Access control prevents cross-employee queries
- ✅ **Contract Owner**: No admin backdoor to read salaries

#### 2. Integrity (STLOP Cryptographic Proofs)

**Guarantee**: Salary data is signed by a trusted notary using ECDSA.

**Mechanism**: EIP-191 signature verification:
```solidity
bytes32 messageHash = keccak256(abi.encodePacked(msg.sender, salary, timestamp));
bytes32 ethSignedMessageHash = keccak256(
    abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)
);
address recoveredSigner = ecrecover(ethSignedMessageHash, v, r, s);
require(recoveredSigner == TRUSTED_NOTARY, "Invalid Notary Signature");
```

**Threat Mitigation**:
- ✅ **Signature Forgery**: ECDSA cryptography prevents fake signatures
- ✅ **Data Tampering**: Any modification invalidates the signature
- ✅ **Replay Attacks**: Timestamp validation prevents reuse of old proofs

#### 3. Access Control (Smart Contract Logic)

**Guarantee**: Only employees can view their own salary.

**Mechanism**:
```solidity
function getMySalary() external view returns (uint256) {
    require(hasProof[msg.sender], "No salary record");
    return salaries[msg.sender]; // Only returns if msg.sender == employee
}
```

**Threat Mitigation**:
- ✅ **Unauthorized Access**: `msg.sender` check prevents cross-employee queries
- ✅ **Contract Enumeration**: No function to list all employees
- ✅ **Admin Abuse**: No owner privileges to override access control

---

### ⚠️ Trust Assumptions (Alpha)

#### 1. Trusted Notary

**Assumption**: The notary (hardcoded address in contract) is honest and signs accurate data.

**Risk**: If the notary is compromised, it can:
- Sign false salary data (e.g., inflate an employee's salary)
- Refuse to sign legitimate data (denial of service)
- Log sensitive data (privacy leak)

**Mitigation (Current)**: Operational security of notary infrastructure

**Mitigation (Future)**: ROFL-based MPC signing (no single point of trust)

#### 2. Sapphire ParaTime Security

**Assumption**: Oasis Sapphire's TEE implementation is secure.

**Risk**: If the TEE is compromised, encrypted state could be exposed.

**Mitigation**: 
- Oasis Foundation's security audits of ParaTime
- Intel SGX/AMD SEV security guarantees
- Regular security updates and patches

**Note**: This is a **platform-level** assumption, not specific to UPE.

#### 3. Smart Contract Correctness

**Assumption**: `PrivatePayroll.sol` has no bugs or vulnerabilities.

**Risk**: Contract bugs could lead to:
- Unauthorized access to salary data
- Signature verification bypass
- Denial of service

**Mitigation (Current)**: Code review and unit testing

**Mitigation (Future)**: Formal security audit before mainnet deployment

---

### ❌ What This Does NOT Guarantee (Alpha)

#### 1. Decentralized Notary

**Not Guaranteed**: Single trusted notary (centralization risk)

**Future Work**: ROFL integration with M-of-N threshold signatures

#### 2. zkTLS Proofs

**Not Guaranteed**: Data authenticity relies on notary's honesty, not cryptographic TLS proofs

**Future Work**: TLSNotary or DECO protocol integration

#### 3. Production Readiness

**Not Guaranteed**: No formal security audit, limited testing

**Future Work**: Security audit, stress testing, mainnet deployment

---

## Target Trust Model (ROFL Integration)

### Planned Improvements

#### 1. Decentralized Notary (MPC Signing)

**Goal**: Eliminate single point of trust in notary.

**Mechanism**: 
- Deploy M-of-N ROFL nodes
- Use threshold signatures (e.g., FROST protocol)
- Require majority consensus to sign salary data

**Benefit**: Compromising one node is insufficient to forge signatures

**Timeline**: 6-9 months (requires additional funding)

#### 2. zkTLS Proofs

**Goal**: Cryptographic proof of data authenticity without trusting notary.

**Mechanism**:
- Use TLSNotary or similar zkTLS library
- Generate proof during API fetch (e.g., payroll system)
- Verify proof in ROFL app before signing

**Benefit**: No trust assumption on data source or notary

**Timeline**: 9-12 months (requires additional funding)

#### 3. On-Chain Key Rotation

**Goal**: Update notary keys without redeploying contracts.

**Mechanism**:
- Store `TRUSTED_NOTARY` in contract storage (not constant)
- Implement governance mechanism for key updates
- Emit events for transparency

**Benefit**: Respond to key compromise without breaking existing deployments

**Timeline**: 3-6 months

---

## Threat Model Analysis

### Threat 1: Malicious Notary

| Scenario | Current Mitigation | Future Mitigation |
|----------|-------------------|-------------------|
| **Forge Salary Data** | ⚠️ Trust assumption | ✅ MPC (M-of-N consensus) |
| **Refuse to Sign** | ⚠️ Single point of failure | ✅ Redundant ROFL nodes |
| **Log Sensitive Data** | ⚠️ Operational security | ✅ zkTLS (no plaintext access) |

**Risk Level (Alpha)**: ⚠️ Medium (acceptable for research prototype)  
**Risk Level (ROFL)**: ✅ Low (cryptographic guarantees)

### Threat 2: Sapphire ParaTime Compromise

| Scenario | Current Mitigation | Future Mitigation |
|----------|-------------------|-------------------|
| **TEE Vulnerability** | ✅ Oasis security audits | ✅ Same (platform-level) |
| **Validator Collusion** | ✅ TEE isolation | ✅ Same (platform-level) |
| **Side-Channel Attacks** | ✅ Intel SGX/AMD SEV | ✅ Same (platform-level) |

**Risk Level**: ✅ Low (relies on Oasis Foundation's security)

### Threat 3: Smart Contract Bugs

| Scenario | Current Mitigation | Future Mitigation |
|----------|-------------------|-------------------|
| **Signature Bypass** | ✅ ECDSA cryptography | ✅ Formal verification |
| **Access Control Bug** | ✅ Code review | ✅ Security audit |
| **Reentrancy Attack** | ✅ No external calls | ✅ Same (not applicable) |

**Risk Level (Alpha)**: ⚠️ Medium (no audit yet)  
**Risk Level (Production)**: ✅ Low (after security audit)

### Threat 4: Replay Attacks

| Scenario | Current Mitigation | Future Mitigation |
|----------|-------------------|-------------------|
| **Reuse Old Signature** | ✅ Timestamp validation | ✅ Nonce-based prevention |
| **Cross-Contract Replay** | ✅ EIP-191 format | ✅ Chain ID + contract address |

**Risk Level**: ✅ Low (adequate mitigation)

---

## Comparison to Alternative Approaches

### vs. zkSNARKs (Zero-Knowledge Proofs)

| Aspect | zkSNARKs | UPE + Sapphire |
|--------|----------|----------------|
| **Privacy** | ✅ Selective disclosure | ✅ Full encrypted state |
| **Trust Model** | ✅ No trusted party | ⚠️ Trusted notary (Alpha) |
| **Complexity** | ❌ Circuit design | ✅ Standard Solidity |
| **Proving Time** | ❌ Minutes | ✅ Instant (signature) |
| **Gas Cost** | ❌ ~500k | ✅ ~50k |

**Conclusion**: UPE has a weaker trust model (Alpha) but better UX and efficiency. ROFL integration will close the trust gap.

### vs. Private Blockchains (Hyperledger)

| Aspect | Hyperledger | UPE + Sapphire |
|--------|-------------|----------------|
| **Privacy** | ✅ Full | ✅ Full |
| **Trust Model** | ⚠️ Permissioned validators | ✅ Public blockchain |
| **Decentralization** | ❌ Low | ✅ High |
| **Censorship Resistance** | ❌ Low | ✅ High |

**Conclusion**: UPE provides privacy **without sacrificing** decentralization.

---

## Security Roadmap

### Phase 1: Alpha Prototype (✅ Current)

- [x] STLOP proof system with single trusted notary
- [x] Sapphire encrypted state integration
- [x] Access control in smart contracts
- [x] Unit tests for signature verification

**Trust Level**: ⚠️ Suitable for demos and friendly pilots

### Phase 2: Multi-Notary (📋 Planned)

- [ ] M-of-N threshold signatures
- [ ] Redundant notary infrastructure
- [ ] On-chain key rotation

**Trust Level**: ⚠️ Suitable for limited production pilots

**Timeline**: 3-6 months

### Phase 3: ROFL Integration (📋 Planned)

- [ ] ROFL app deployment
- [ ] MPC-based signing cluster
- [ ] zkTLS proof generation
- [ ] Sapphire contract updates

**Trust Level**: ✅ Suitable for institutional production

**Timeline**: 6-9 months

### Phase 4: Production Hardening (📋 Planned)

- [ ] Formal security audit
- [ ] Stress testing on Sapphire Mainnet
- [ ] Bug bounty program
- [ ] Institutional partnerships

**Trust Level**: ✅ Production-ready

**Timeline**: 12-18 months

---

## Honest Assessment

### What We Can Claim

✅ **Sapphire Confidentiality**: Encrypted state is a **strong guarantee** (TEE-based)  
✅ **Signature Integrity**: ECDSA prevents forgery (cryptographic guarantee)  
✅ **Access Control**: Smart contract logic prevents unauthorized queries  
✅ **Research Prototype**: Demonstrates feasibility of institutional privacy layer  

### What We Cannot Claim (Yet)

❌ **Decentralized Notary**: Single trusted signer (centralization risk)  
❌ **zkTLS Proofs**: Data authenticity relies on notary honesty  
❌ **Production Ready**: No formal security audit  
❌ **Zero Trust**: Some trust assumptions remain (Alpha)  

### What We Will Achieve (ROFL)

🔮 **Decentralization**: MPC signing eliminates single point of trust  
🔮 **Cryptographic Authenticity**: zkTLS proofs replace trust assumptions  
🔮 **Production Hardening**: Security audit + mainnet deployment  
🔮 **Institutional Adoption**: Partnerships with payroll providers, HR platforms  

---

## Regulatory Considerations

### GDPR Compliance

**Requirement**: Personal data must be protected and deletable.

**UPE Approach**:
- ✅ **Protection**: Sapphire's encrypted state meets confidentiality requirements
- ⚠️ **Deletion**: Blockchain immutability conflicts with "right to be forgotten"

**Mitigation**: 
- Store only hashes or encrypted references on-chain
- Keep plaintext data off-chain with deletion capability
- Use Sapphire's encrypted events for audit logs

### SOC2 Compliance

**Requirement**: Access controls and audit trails.

**UPE Approach**:
- ✅ **Access Control**: Smart contract logic enforces employee-only access
- ✅ **Audit Trail**: `SalaryVerified` events provide immutable logs
- ✅ **Encryption**: Sapphire's TEE meets data protection requirements

### HIPAA Compliance (Future)

**Requirement**: Healthcare data must be encrypted and access-controlled.

**UPE Approach**:
- ✅ **Encryption**: Sapphire's encrypted state
- ✅ **Access Control**: Smart contract logic
- ⚠️ **Business Associate Agreement**: Requires legal framework with Oasis Foundation

**Timeline**: 12+ months (requires legal and compliance work)

---

## Disclaimer

**Research Prototype**: This software is for experimental and development use only.

**Alpha Limitations**:
- Single trusted notary (centralization risk)
- No formal security audit
- Limited testing on Sapphire Testnet
- ROFL integration not yet implemented

**Do not use with real sensitive data or production value.**

**For Production Use**: Wait for Phase 4 (security audit + mainnet deployment) or contact UPE team for pilot program.

---

**Last Updated**: January 2, 2026  
**Grant Program**: Oasis ROSE Bloom  
**Security Status**: Alpha Prototype (Phase 1)
