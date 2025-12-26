# Future Work & Research Roadmap

## Overview

This document outlines the research and development roadmap for the Universal Privacy Engine.
It distinguishes between the currently implemented "Alpha" features and the targeted "Production" capabilities.

The current codebase is a **Research Prototype** dealing with:
1.  **Modular Architecture**: Proving backend abstraction.
2.  **Recorded Data verification**: Deterministic fixture verification.
3.  **Client-Side Execution**: Developer-centric proving flow.

---

## 1. zkTLS (High Priority)

The current "Recorded zkTLS" system is a fixture-based verification layer used to demonstrate the *flow* of data authenticity without the heavy cryptographic overhead of full zkTLS generation during development.

### Implementation Goals
- **TLSNotary Integration**: Replace recorded fixtures with live, 3-party TLS connection proofs (Prover, Notary, Verifier).
- **Circuit Integration**: Bind the SHA256 hash of the verified TLS response directly into the SP1 ZK-VM circuit input.
- **Freshness Guarantees**: Enforce strict timestamp windows on-chain to prevent replay attacks.
- **Selective Disclosure**: Allow revealing only specific JSON fields (e.g., "balance > 1000") without exposing the full response body.

### Research Questions
- How to efficiently verify TLSNotary signatures inside a RISC-V zkVM?
- Minimizing notary trust assumptions via multi-notary aggregation.

---

## 2. Trusted Execution Environments (Optional Hardening)

TEE support is planned as an **optional** hardening layer for operators who require defense-in-depth for private key management or side-channel protection.

### Implementation Goals
- **Hardware Isolation**: Move the `PrivacyEngine` execution inside Intel SGX (via Gramine/Occlum) or AWS Nitro Enclaves.
- **Remote Attestation**: Integrate DCAP (Data Center Attestation Primitives) verification on-chain.
- **Sealed Storage**: Persist long-term secrets (like signing keys) using hardware-derived keys.

### Trade-offs & Analysis
TEEs are not a "silver bullet" and introduce new trust assumptions (Trusting Intel/AWS). They will be treated as a pluggable backend adapter (`adapters/tee`) rather than a mandatory requirement.

---

## 3. On-Chain Verification

### Implementation Goals
- **Universal Verifier**: A single Solidity entrypoint that routes verification to different backends (SP1, Groth16, etc.).
- **Gas Optimization**: Batch verification of multiple proofs to reduce L1 costs.
- **Cross-Chain State**: Experimenting with Hyperlane/Wormhole for posting proofs to non-EVM chains.

---

## Non-Goals

- **Enterprise Compliance**: This project does not currently aim for SOC2 or HIPAA compliance.
- **Hardware Manufacturing**: We will not build custom hardware; we rely on commodity TEEs.
