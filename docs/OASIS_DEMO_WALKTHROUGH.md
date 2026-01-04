# Oasis Sapphire End-to-End Demo Flow

> **⚠️ DEPLOYMENT STATUS: Local Network Testing**  
> This demo currently runs on **local Hardhat network** for development and testing.  
> **Sapphire Testnet deployment** is planned for the next phase of the grant.  
> All contract logic and cryptographic verification work correctly on local network.

## Architecture

The **Universal Privacy Engine (UPE)** enables private data settlement on Oasis Sapphire through the following flow:

1.  **Notary (Off-Chain)**: Captures web2 data (e.g., payroll API), hashes it, and signs `(User, Data, Timestamp)` using an EIP-191 compatible wallet. This creates a **STLOP Proof**.
2.  **Sapphire Contract (On-Chain)**: The `PrivatePayroll` contract receives the proof. It:
    *   Verifies the Notary's signature.
    *   Validates the timestamp.
    *   Stores the data in **Sapphire Encrypted State**.
3.  **User View (Decrypted)**: The user interacts with the contract to retrieve their own data. Because Sapphire runs in a TEE, only the designated user can decrypt their own state.

```mermaid
graph LR
    N[Notary (Off-Chain)] -- Signed Proof --> C[PrivatePayroll.sol (Sapphire)]
    C -- Encrypted State --> S[(Sapphire Storage)]
    S -- Decrypted View --> U[User (Employee)]
```

## How to Run

1.  **Install Dependencies**:
    ```bash
    cd contracts/oasis
    npm install
    ```

2.  **Run the Demo Script**:
    ```bash
    npx hardhat run scripts/demo_sapphire_flow.js
    ```
    *(Note: For Sapphire Testnet, you would add `--network sapphire_testnet`)*

## Key Concept: Input Visibility vs. Storage Privacy

It is important to understand the difference between **transaction inputs** and **storage state**:

*   **Transaction Inputs**: When sending the proof, the `salary` value is visible in the transaction data (mempool/block).
    *   *Mitigation*: In a full production deployment, users would use the **Sapphire Wrapper** (Oasis SDK) to encrypt the transaction inputs *before* they reach the network, ensuring end-to-end privacy.
*   **Storage State**: Once stored in the `mapping(address => uint256) private salaries`, the data is **fully encrypted** by the Sapphire ParaTime key. It cannot be read by node operators or via `getStorageAt`.

This demo focuses on demonstrating the **Encrypted Storage** and **Access Control** capabilities.

---

## Merged Demo Notes (from DEMO.md)

### What This Demo Proves

#### ✅ Technical Capabilities Demonstrated

1. **STLOP Proof Verification**
   - EIP-191 signature verification works on-chain
   - Notary trust anchor enforcement is functional
   - Timestamp validation prevents replay attacks

2. **Sapphire Encrypted State**
   - Private mappings are cryptographically encrypted at ParaTime level
   - Access control ensures only employees can view their own data
   - State queries work correctly with encrypted storage

3. **Cross-Environment Compatibility**
   - Standard Solidity contracts work on Sapphire
   - Hardhat tooling is fully compatible
   - No custom compilers or frameworks required

### ❌ What This Demo Does NOT Prove

1. **Data Authenticity**
   - No proof that data came from a legitimate payroll source
   - Signatures are from a test notary (not institutional)
   - No zkTLS or authenticated data ingestion

2. **Institutional Trustworthiness**
   - No KYC or identity verification
   - No legal entity validation
   - No institutional partnerships

3. **Production Readiness**
   - No security audit
   - No formal verification
   - No production hardening
   - Single trusted notary (centralization risk)

4. **Regulatory Compliance**
   - No legal framework
   - No regulatory approval
   - No compliance with financial regulations

### Research Context

This demo is part of early-stage research into:

1. **Privacy-Preserving Data Settlement**: Can we settle sensitive off-chain data on-chain without exposure?
2. **Sapphire Integration**: How do we leverage Confidential EVM for institutional use cases?
3. **STLOP Methodology**: Are lightweight signed proofs sufficient for Phase 1?

**This is NOT**:
- A production system
- An institutional product
- A regulatory solution
- A finished protocol

### Next Steps (Research Directions)

#### Short-term (Grant Scope)
- Deploy to Sapphire Testnet
- Create demo video showing full flow
- Document deployment steps
- Benchmark gas costs

#### Medium-term (Post-Grant)
- Multi-notary support (M-of-N signatures)
- Additional use cases (compliance, financial statements)
- Developer SDK for STLOP proof generation
- Preliminary security review

#### Long-term (ROFL Integration)
- Decentralized notary with MPC signing
- zkTLS proofs for data authenticity
- Production hardening and formal audit
- Institutional partnerships

### Disclaimer

This is experimental research software. It has NOT been audited, is NOT production-ready, and should NOT be used for any real-world financial or compliance purposes.

**No institutional partnerships, users, or real-world deployments exist.**

Use at your own risk.
