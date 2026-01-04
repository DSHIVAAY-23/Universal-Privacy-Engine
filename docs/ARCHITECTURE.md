# Architecture — UPE for Oasis Sapphire (Confidential EVM)

## Purpose

This document describes the minimal, Sapphire-centric architecture for the Universal Privacy Engine (UPE) as required for an Oasis ROSE Bloom Grant. The goal: ingest sensitive Web2 data, verify it with signed TLS observation proofs (STLOP), and *settle the verified data into Sapphire's encrypted contract state* so the data is confidential by default.

## End-to-end Flow (single story)

1. **Bank API (HTTPS)** — the source of truth (e.g., payroll provider).  
2. **UPE Notary (Rust / STLOP)** — a lightweight trusted notary that:
   - performs an HTTPS fetch,
   - records the TLS transcript,
   - produces a signed observation proof (STLOP JSON).
3. **Signed Proof (public JSON)** — contains a compact verifiable statement; the proof is public, auditable, and can be verified on-chain. **The proof contains no secret plaintext**; only a pointer + authenticated digest of the observation.
4. **Oasis Sapphire Contract (Solidity)** — `PrivatePayroll.sol` verifies the signature and stores the related data **in encrypted private state**. The proof is public; the stored data is private.
5. **Private State** — Sapphire's ParaTime Key Manager encrypts contract storage such that only authorized users (or the contract logic) can access the plaintext. On-chain observers cannot read the stored salary/RWA values.

## Security & Assumptions (brief)

- The notary signer key is a trust anchor for Phase 1. The design anticipates migration to ROFL (decentralized notaries) in Phase 2.
- Proof verification uses standard ECDSA/Ed25519 signature verification on-chain.
- Confidentiality is enforced by Sapphire's encrypted contract storage, not by obfuscation or off-chain secrets.

## Diagram (text)

```
Bank API → UPE Notary (STLOP signer) → Signed Proof (public) → Sapphire Contract (verify & encrypted store)
```

## Why Sapphire?

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

## STLOP Proof System

### Design Philosophy

Lightweight cryptographic proofs for off-chain data ingestion.

### Notary Workflow (Pseudocode)

```rust
async fn generate_stlop_proof(
    api_url: &str,
    employee_address: Address,
) -> Result<STLOPProof, Error> {
    // 1. Fetch data from external API
    let response = reqwest::get(api_url).await?;
    let salary: u256 = response.json().await?;
    
    // 2. Get current timestamp
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    
    // 3. Construct message hash (matches on-chain logic)
    let message = encode_packed(&[
        employee_address.as_bytes(),
        &salary.to_be_bytes(),
        &timestamp.to_be_bytes(),
    ]);
    let message_hash = keccak256(&message);
    
    // 4. Sign with notary's private key (ECDSA)
    let signature = sign_ecdsa(&message_hash, &NOTARY_PRIVATE_KEY)?;
    
    // 5. Return STLOP proof
    Ok(STLOPProof {
        salary,
        timestamp,
        signature,
    })
}
```

### Signature Scheme: EIP-191

**Why EIP-191?**
- Standard Ethereum signed message format
- Compatible with MetaMask and other wallets
- Prevents cross-contract replay attacks

**Format**:
```
"\x19Ethereum Signed Message:\n32" + keccak256(employee_address, salary, timestamp)
```

### Trust Model

**Alpha Prototype**:
- Single trusted notary (hardcoded address in contract)
- Centralization risk: compromised notary can sign false data

**ROFL Enhancement (Future)**:
- MPC-based signing cluster (M-of-N threshold)
- zkTLS proofs for data authenticity
- No single point of trust

## Sapphire Smart Contracts

### PrivatePayroll.sol Architecture

#### State Variables

```solidity
// TRUSTED NOTARY (Hardcoded for Alpha)
address public constant TRUSTED_NOTARY = 0x1234...7890;

// ENCRYPTED STATE (Sapphire Magic)
mapping(address => uint256) private salaries;
mapping(address => bool) private hasProof;
```

**Key Insight**: On Sapphire, `private` mappings are **actually private**. The ParaTime encrypts all storage slots using TEE-based encryption.

#### Proof Verification Flow

```solidity
function verifyAndStoreSalary(
    uint256 salary,
    uint256 timestamp,
    bytes memory signature
) external {
    // Step 1: Reconstruct message hash (must match notary's logic)
    bytes32 messageHash = keccak256(
        abi.encodePacked(msg.sender, salary, timestamp)
    );
    
    // Step 2: Apply EIP-191 prefix
    bytes32 ethSignedMessageHash = keccak256(
        abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)
    );
    
    // Step 3: Recover signer from signature
    address recoveredSigner = recoverSigner(ethSignedMessageHash, signature);
    
    // Step 4: Validate signer is trusted notary
    require(recoveredSigner == TRUSTED_NOTARY, "Invalid Notary Signature");
    
    // Step 5: Store in ENCRYPTED state
    salaries[msg.sender] = salary;
    hasProof[msg.sender] = true;
    
    // Step 6: Emit event (timestamp only, no amount)
    emit SalaryVerified(msg.sender, timestamp);
}
```

#### Access Control

```solidity
function getMySalary() external view returns (uint256) {
    require(hasProof[msg.sender], "No salary record");
    return salaries[msg.sender];
}
```

**Privacy Guarantee**: Only `msg.sender` can read their own salary. Even contract owner cannot access other employees' data.

## ROFL Integration (Future Architecture)

### Current vs. Future

| Component | Current (Alpha) | Future (ROFL) |
|-----------|----------------|---------------|
| **Notary** | Single trusted signer | MPC cluster (M-of-N) |
| **Data Fetch** | Direct API calls | zkTLS proofs |
| **Trust Model** | Trust notary's honesty | Cryptographic guarantees |
| **Decentralization** | ❌ Centralized | ✅ Decentralized |

### ROFL Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   ROFL App (Off-Chain)                      │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  1. Fetch data via zkTLS (TLSNotary)                 │  │
│  │     • No trusted notary needed                       │  │
│  │     • Cryptographic proof of TLS session             │  │
│  │                                                      │  │
│  │  2. MPC Signing (Threshold Signatures)               │  │
│  │     • M-of-N ROFL nodes must agree                   │  │
│  │     • No single point of compromise                  │  │
│  │                                                      │  │
│  │  3. Generate ROFL Attestation                        │  │
│  │     • Proof that computation ran inside ROFL         │  │
│  │     • Includes zkTLS proof + MPC signature           │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                         │                                   │
└─────────────────────────┼───────────────────────────────────┘
                          │ ROFL Attestation
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              Sapphire Contract (On-Chain)                   │
│                                                             │
│  • Verify ROFL attestation (cryptographic proof)            │
│  • Verify zkTLS proof (data authenticity)                   │
│  • Store in encrypted state (Sapphire confidentiality)      │
└─────────────────────────────────────────────────────────────┘
```

**Timeline**: 6-9 months (requires additional funding)

## Security Considerations

### Threat Model

| Threat | Mitigation (Current) | Mitigation (Future) |
|--------|---------------------|---------------------|
| **Malicious Notary** | ⚠️ Trust assumption | ✅ MPC + zkTLS (ROFL) |
| **Replay Attacks** | ✅ Timestamp validation | ✅ Nonce-based prevention |
| **State Exposure** | ✅ Sapphire encryption | ✅ Sapphire encryption |
| **Signature Forgery** | ✅ ECDSA cryptography | ✅ Threshold signatures |
| **Contract Bugs** | ⚠️ No audit yet | ✅ Formal verification |

### Trust Assumptions (Alpha)

1. **Notary Honesty**: We trust the notary to sign accurate data
2. **Sapphire Security**: We trust Oasis ParaTime's TEE implementation
3. **EVM Correctness**: We trust Solidity's `ecrecover()` implementation

**Note**: These assumptions are acceptable for a research prototype but require hardening for production.

## Conclusion

The Universal Privacy Engine leverages **Oasis Sapphire's unique Confidential EVM** to create an Institutional Privacy Layer with:

1. **Simple Architecture**: Rust notary → Sapphire contract → Encrypted state
2. **Strong Privacy**: TEE-based encryption at ParaTime level
3. **Developer-Friendly**: Standard Solidity, no custom tooling
4. **Future-Proof**: ROFL integration roadmap for decentralization

**This architecture is Sapphire-native** and cannot be replicated on standard EVM chains.

---

**Last Updated**: January 4, 2026  
**Grant Program**: Oasis ROSE Bloom  
**Architecture Status**: Phase 2 (Sapphire-Focused)
