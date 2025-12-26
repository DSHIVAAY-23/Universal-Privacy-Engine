# Trust Model

## Overview

This document explains the security assumptions, trust requirements, and roadmap for the Universal Privacy Engine. It provides an honest assessment of the current state (Alpha) and the target production architecture.

---

## Current State (Alpha Research Prototype)

### ⚠️ Trust Assumptions

#### 1. **Client-Side Proving**

**Current Implementation**:
- Prover runs on the user's local machine
- No hardware isolation or secure enclaves
- Operator/developer can access raw input data

**Security Implications**:
- ❌ **No privacy from operator**: If the prover is compromised, all data is visible
- ❌ **No tamper resistance**: Malicious operator can modify prover code
- ❌ **Trust required**: Users must trust the software and execution environment

**Use Cases**:
- ✅ Development and testing
- ✅ Proof-of-concept demonstrations
- ✅ Research prototypes
- ❌ Production deployments with sensitive data

#### 2. **Mock TEE Adapter**

**Current Implementation**:
- `TeeProverStub` simulates TEE attestations
- Uses Ed25519 signatures (not hardware-backed)
- No real Intel SGX or AWS Nitro integration

**Security Implications**:
- ❌ **No hardware security**: Attestations are software-generated
- ❌ **No remote attestation**: Cannot prove execution environment
- ❌ **Mock only**: Suitable for development, not production

**Code Warning**:
```rust
/// @dev ⚠️ WARNING: This is a MOCK verifier for development only!
/// @dev DO NOT deploy to production. Use real SP1 verifier instead.
/// @dev Mock verifier does not perform actual cryptographic verification.
```

#### 3. **No zkTLS Integration**

**Current Implementation**:
- `HttpProvider` fetches data over HTTPS
- `verify_tls_signature()` always returns `true`
- No cryptographic proof of data authenticity

**Security Implications**:
- ❌ **Trust HTTPS endpoints**: Must trust API servers not to lie
- ❌ **No proof of data source**: Cannot prove data came from specific server
- ❌ **Vulnerable to MITM**: If TLS is compromised, data can be tampered

**Trust Requirement**:
- Users must trust:
  1. The HTTPS endpoint (e.g., bank API)
  2. The TLS certificate authority
  3. The network path (no MITM attacks)

---

## Target State (Production Architecture)

### ✅ Hardware-Backed TEE Integration

**Goal**: Run the prover inside a Trusted Execution Environment

**Supported TEEs**:
1. **Intel SGX** (Software Guard Extensions)
2. **AWS Nitro Enclaves**
3. **Azure Confidential Computing**

**Security Guarantees**:
- ✅ **Hardware isolation**: Prover runs in encrypted memory
- ✅ **Remote attestation**: Users can verify execution environment
- ✅ **Tamper resistance**: Operator cannot access raw data
- ✅ **Sealed storage**: Secrets protected by hardware keys

**Trust Model**:
```
User trusts:
  1. Hardware vendor (Intel, AMD, AWS)
  2. TEE firmware (verified via attestation)
  3. Prover code (open-source, audited)

User does NOT trust:
  ❌ Cloud provider (AWS, Azure, GCP)
  ❌ Operator/developer
  ❌ Operating system
```

**Attestation Flow**:
```
1. User requests proof
2. TEE generates attestation (DCAP quote)
3. User verifies attestation:
   - Check enclave measurement (hash of prover code)
   - Verify signature from hardware
   - Confirm timestamp freshness
4. If valid, user trusts the proof
```

### ✅ zkTLS Integration

**Goal**: Cryptographically prove data came from specific HTTPS endpoint

**Supported Protocols**:
1. **TLSNotary** (3-party TLS with notary)
2. **DECO** (2-party TLS with MPC)

**Security Guarantees**:
- ✅ **Data authenticity**: Cryptographic proof of TLS session
- ✅ **Selective disclosure**: Prove specific fields without revealing full response
- ✅ **Verifiable timestamps**: Prevent replay attacks
- ✅ **No trust in endpoint**: Even if API lies, proof will fail

**Trust Model**:
```
User trusts:
  1. TLS certificate authority (standard web PKI)
  2. Notary (for TLSNotary) or MPC protocol (for DECO)
  3. Cryptographic primitives (TLS 1.3, signatures)

User does NOT trust:
  ❌ API endpoint (can lie, but proof will fail)
  ❌ Network path (MITM attacks detectable)
```

**zkTLS Flow**:
```
1. Prover initiates TLS session with API
2. Notary participates in handshake (TLSNotary)
   OR MPC splits session keys (DECO)
3. Prover receives HTTP response
4. Prover generates ZK proof of response content
5. Notary signs commitment (TLSNotary only)
6. User verifies:
   - Notary signature
   - TLS certificate chain
   - Timestamp freshness
   - ZK proof validity
```

---

## Comparison: Current vs. Target

| Aspect | Current (Alpha) | Target (Production) |
|--------|-----------------|---------------------|
| **Prover Isolation** | None (client-side) | Hardware TEE (SGX/Nitro) |
| **Data Privacy** | ❌ Visible to operator | ✅ Encrypted in enclave |
| **Attestation** | ❌ Mock (Ed25519) | ✅ Hardware-backed (DCAP) |
| **Data Authenticity** | ❌ Trust HTTPS | ✅ zkTLS proof |
| **Tamper Resistance** | ❌ Software only | ✅ Hardware-enforced |
| **Remote Verification** | ❌ Not possible | ✅ Attestation verification |
| **Trust Requirement** | High (trust operator) | Low (trust hardware vendor) |

---

## Security Roadmap

### Phase 1: TEE Integration (6-9 months)

**Deliverables**:
- [ ] Intel SGX adapter with DCAP attestation
- [ ] AWS Nitro Enclaves integration
- [ ] Azure Confidential Computing support
- [ ] Remote attestation verification library
- [ ] Sealed storage for secrets

**Security Improvements**:
- ✅ Hardware isolation for prover
- ✅ Remote attestation
- ✅ Encrypted memory
- ⚠️ Still trusts HTTPS endpoints (no zkTLS yet)

### Phase 2: zkTLS Integration (9-12 months)

**Deliverables**:
- [ ] TLSNotary proof generation during HTTP fetch
- [ ] DECO protocol integration
- [ ] Selective disclosure for privacy
- [ ] On-chain proof verification
- [ ] Notary infrastructure

**Security Improvements**:
- ✅ Cryptographic data authenticity
- ✅ No trust in API endpoints
- ✅ Verifiable timestamps
- ✅ Selective disclosure

### Phase 3: Production Hardening (12-18 months)

**Deliverables**:
- [ ] Security audit (smart contracts + Rust)
- [ ] Formal verification of critical paths
- [ ] Bug bounty program
- [ ] Incident response plan
- [ ] Key management infrastructure

**Security Improvements**:
- ✅ Audited codebase
- ✅ Formally verified components
- ✅ Battle-tested in production

---

## Threat Model

### Threats Mitigated (Target State)

1. **Malicious Operator**:
   - ✅ Cannot access raw data (TEE isolation)
   - ✅ Cannot tamper with prover (attestation)
   - ✅ Cannot forge proofs (hardware keys)

2. **Compromised API**:
   - ✅ Cannot provide fake data (zkTLS proof)
   - ✅ Cannot replay old data (timestamp verification)

3. **Man-in-the-Middle**:
   - ✅ Cannot tamper with TLS session (zkTLS)
   - ✅ Cannot inject fake data (cryptographic proof)

### Threats NOT Mitigated

1. **Hardware Vendor Compromise**:
   - ❌ If Intel/AMD/AWS is malicious, TEE security fails
   - Mitigation: Use multiple TEE vendors, compare attestations

2. **Side-Channel Attacks**:
   - ❌ Spectre, Meltdown, cache timing attacks
   - Mitigation: Use latest CPU microcode, constant-time crypto

3. **Denial of Service**:
   - ❌ Operator can refuse to generate proofs
   - Mitigation: Decentralized prover network

4. **Social Engineering**:
   - ❌ User tricked into trusting wrong attestation
   - Mitigation: User education, wallet integration

---

## Recommendations for Current Use

### ✅ Acceptable Use Cases (Alpha)

1. **Development and Testing**:
   - Building applications
   - Testing proof generation
   - Benchmarking performance

2. **Proof-of-Concept Demonstrations**:
   - Grant proposals
   - Technical demos
   - Research papers

3. **Non-Sensitive Data**:
   - Public data verification
   - Educational examples
   - Testnet deployments

### ❌ Unacceptable Use Cases (Alpha)

1. **Production Deployments**:
   - Real user data
   - Financial applications
   - Healthcare records

2. **Sensitive Data**:
   - Private keys
   - Passwords
   - Personal information

3. **High-Value Assets**:
   - Cryptocurrency custody
   - Financial instruments
   - Legal documents

---

## Disclaimer

**⚠️ ALPHA SOFTWARE WARNING**

This is research software. The current trust model is **NOT suitable for production use**. Do not process sensitive data or deploy to mainnet without:

1. ✅ Real TEE integration (SGX/Nitro)
2. ✅ zkTLS data authenticity
3. ✅ Security audit
4. ✅ Formal verification
5. ✅ Bug bounty program

**Use at your own risk.**

---

## References

- **Intel SGX**: [SGX Developer Guide](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/overview.html)
- **AWS Nitro**: [Nitro Enclaves Documentation](https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave.html)
- **TLSNotary**: [TLSNotary Protocol](https://tlsnotary.org/)
- **DECO**: [DECO Whitepaper](https://arxiv.org/abs/1909.00938)
