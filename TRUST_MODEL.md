# Trust Model

## Overview

This document defines the security assumptions for the **Universal Privacy Engine**.
It explicitly separates the **Current State (Alpha)** from the **Target State (Future Work)** to avoid ambiguity during review.

---

## 1. Current Trust Model (Implemented)

The Alpha version enables developers to build and test the *architecture* of privacy-preserving applications.
It does **NOT** yet provide full cryptographic confidentiality or hardware-backed integrity.

### ✅ What is Secured
- **Execution Integrity**: The SP1 zkVM cryptographically proves that the Rust guest code was executed correctly on the provided inputs.
- **Data Integrity (Static)**: The "Recorded zkTLS" system proves that input data matches a specific, pre-recorded SHA256 hash of a TLS session.
- **Deterministic Verification**: All verification logic (proof check, fixture check) is deterministic and reproducible.

### ⚠️ Trust Assumptions (Current)
1.  **Prover Visibility**: The entity running the prover (the User or Developer) has full view of the private inputs. There is no TEE isolation in the Alpha.
2.  **HTTPS Endpoint Trust**: We assume the data source (e.g., specific API domain) provides correct data.
3.  **Fixture Trust**: In "Recorded" mode, the verifier trusts that the fixture metadata corresponds to a valid historical event.

### ❌ What This Does NOT Guarantee
- **Confidential Computing**: The operator of the node *can* see the data.
- **Live TLS Proofs**: The system currently verifies *recorded* evidence, not live TLS handshakes.
- **Hardware Security**: No dependence on Intel SGX/AWS Nitro in the current version.

---

## 2. Target Trust Model (Future Work)

The roadmap aims to reduce trust assumptions by integrating cryptographic multi-party computation and hardware isolation.

### Planned Improvements
1.  **zkTLS (TLSNotary / DECO)**:
    - **Goal**: Verify data authenticity without trusting the TLS provider fully.
    - **Mechanism**: Use 3-party MPC (User, Server, Notary) to prove data origin.
    - **Benefit**: "Don't Verify, Verify".

2.  **Optional TEE Isolation**:
    - **Goal**: Hide private inputs from the node operator.
    - **Mechanism**: Run the `PrivacyEngine` inside an enclave (SGX/Nitro).
    - **Benefit**: Defense-in-depth for key management.

---

## 3. Threat Model (Alpha)

| Threat | Mitigated? | Explanation |
|--------|------------|-------------|
| **Malicious Modification of Logic** | ✅ Yes | SP1 Proof ensures code integrity. |
| **Tampered Input Data** | ✅ Yes | Recorded zkTLS verifies SHA256 hash of inputs. |
| **Malicious Node Operator** | ❌ No | Operator can see plaintext inputs (Alpha limitation). |
| **MITM on Data Fetch** | ❌ No | Relies on standard HTTPS trust currently. |

---

## Disclaimer

This software is a **Research Prototype**.
Do not use for production keys or high-value assets.
The "Privacy" in the name refers to the *capability* of the architecture to handle private inputs in future versions, not a guarantee of confidentiality in the current v0.1 release.
