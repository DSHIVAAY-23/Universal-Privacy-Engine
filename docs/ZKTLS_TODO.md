# zkTLS Integration TODOs

This document tracks the missing zkTLS (Zero-Knowledge Transport Layer Security) features required for the data ingestion layer.

## Overview
Currently, the `HttpProvider` fetches data over standard HTTPS but does not verify the authenticity of the data source cryptographically beyond the standard TLS session. This means the host could modify the data before feeding it into the Privacy Engine.

## Missing Features

| File Path | Component | Missing Cryptographic Proof | Intended Protocol |
|-----------|-----------|-----------------------------|-------------------|
| `core/src/data_source/http.rs` | `verify_tls_signature` | TLS Session Proof (Notary Signature) | TLSNotary |
| `core/src/data_source/http.rs` | `verify_tls_signature` | Selective Disclosure (JSON Field Proof) | TLSNotary / DECO |

## Implementation Plan

### Phase 1: TLSNotary Integration (Off-Chain)
- [ ] Integrate `tlsn` crate (Prover/Verifier).
- [ ] Instrument `HttpProvider` to capture TLS session.
- [ ] Implement `verify_tls_signature` to check Notary signature.

### Phase 2: On-Chain Verification
- [ ] Wrap TLS verification logic in a ZK circuit (SP1).
- [ ] Expose valid TLS session as a public input to the Privacy Engine.

## References
- [TLSNotary](https://tlsnotary.org)
- [DECO](https://www.deco.works)
