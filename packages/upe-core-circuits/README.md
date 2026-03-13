# upe-core-circuits

**Core cryptographic IP. Circuit generation logic is kept private during audit phase.**

This package contains the universal zero-knowledge circuit library for the Universal Privacy Engine.

## Contents

- **Circom** — R1CS constraint systems for TLS session transcript verification
- **Halo2** — PLONKish arithmetization circuits for recursive proof composition
- **ACIR/Noir** — Aztec Cairo Intermediate Representation for cross-chain portability

## Status

🔒 **Audit in Progress** — circuit sources are undergoing third-party cryptographic audit.
Public release expected post-audit. Verifier contracts are available in `/adapters/`.

## Architecture

```
upe-core-circuits/
├── circom/          # Circom R1CS circuits
├── halo2/           # Halo2/PLONKish circuits  
├── noir/            # Noir/ACIR circuits
└── shared/          # Shared constraint utilities
```
