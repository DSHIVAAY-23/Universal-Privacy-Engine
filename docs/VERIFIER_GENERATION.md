# Smart Contract Verifier Generation

This guide explains how to generate Solidity verifier contracts for your SP1 zkVM programs.

## Overview

The `generate_verifier` tool automates the creation of chain-specific verifier contracts.
It takes your program's Verification Key (VKey) and produces a `UniversalVerifier.sol` contract ready for deployment.

## Prerequisites

- Rust toolchain
- `contracts/templates` directory must exist (it contains the base logic)

## Usage

### Basic Command

```bash
cargo run --bin generate_verifier -- \
  --program-vkey <YOUR_VKEY_HEX> \
  --out-dir contracts/generated
```

### Arguments

- `--program-vkey`: The 32-byte verification key of your guest program (hex string).
- `--out-dir`: The directory where the Solidity file will be written. Default: `contracts/generated`.
- `--force`: Overwrite existing files without prompting.

### Example

```bash
# Generate verifier for a specific program hash
cargo run --bin generate_verifier -- \
  --program-vkey 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --out-dir contracts/generated \
  --force
```

## Output

The tool generates `UniversalVerifier.sol` which contains:
- The embedded Verification Key.
- Logic to verify proofs against that specific key.
- Integration with the central `SP1Verifier` gateway.

## Testing

To verify the generator itself works:

```bash
cargo test --bin generate_verifier
```
