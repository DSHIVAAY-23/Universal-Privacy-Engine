# Signed TLS Fixtures (Pre-zkTLS) System

This document explains the "Signed TLS Fixtures" verification layer used in the current alpha version.

## Overview
The `RecordedTlsProof` system replaces "blind trust" with a deterministic fixture-based verification flow.
Instead of an insecure stub that always returns `true`, the `HttpProvider` now cryptographically verifies that a JSON response matches a specific, previously recorded TLS session from the target domain.

## What This Proves
- **Integrity**: The JSON data has not been modified since it was recorded.
- **Authenticity**: The data originated from the claim domain (e.g., `example.com`), signed by a specific certificate chain.
- **Traceability**: We verify the SHA256 hash of the certificate chain and the response body.

## What This Does NOT Prove (Yet)
- **Recency**: Since the proof is a static recording, we temporarily allow older timestamps for demo purposes.
- **Privacy (Zero-Knowledge)**: This implementation handles raw data directly. The "Zero-Knowledge" component comes in Phase 2 when this verification logic moves inside an SP1 zkVM circuit (TLSNotary).

## Integration Roadmap
1.  **Phase 1 (Current)**: Fixture-based verification using `RecordedTlsProof`.
2.  **Phase 2**: Integrate `tlsn` (TLSNotary) to generate *fresh* proofs for every request.
3.  **Phase 3**: Move verification on-chain using the `Groth16` wrapper.

## How to Test
Run the specific verification tests:
```bash
cargo test -p universal-privacy-engine-core --test zktls_verification
```
