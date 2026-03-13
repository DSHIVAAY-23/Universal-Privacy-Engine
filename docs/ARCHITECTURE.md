# Architecture — Universal Privacy Engine

## What it does

UPE takes sensitive Web2 data, verifies it with a cryptographic proof, and writes the result into Oasis Sapphire's TEE-encrypted contract state. The proof is public. The underlying data is not.

---

## Phase 1: The STLOP Pipeline (live on testnet)

```
Browser Wallet
    │
    ▼
React Frontend (Vercel)
    │  POST /api/generate-proof  { employee_address }
    ▼
Rust Notary  ──── fetches salary (simulated in Phase 1)
    │  { salary, timestamp, ECDSA signature }
    ▼
PrivatePayroll.sol (Sapphire Testnet)
    │  ecrecover → verify notary sig → write to encrypted mapping
    ▼
getMySalary() — only msg.sender can read it
```

### ECDSA signing (EIP-191)

The Notary constructs the proof message to match what the contract will reconstruct on-chain:

```rust
let message = keccak256(abi.encodePacked(employee_address, salary, timestamp));
let eth_message = keccak256("\x19Ethereum Signed Message:\n32" ++ message);
let signature = sign_secp256k1(eth_message, notary_key);
```

```solidity
bytes32 hash = keccak256(abi.encodePacked(msg.sender, salary, timestamp));
bytes32 ethHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
address signer = ECDSA.recover(ethHash, signature);
require(signer == TRUSTED_NOTARY);
salaries[msg.sender] = salary; // TEE-encrypted at rest
```

### Why Sapphire

On Ethereum, `private` mappings are readable via `eth_getStorageAt`. Anyone with an archive node can pull your salary out of a "private" contract. On Sapphire, all storage is encrypted by the TEE — not by the application, by the protocol. Even validators can't read it.

```bash
# Ethereum: readable by anyone
cast storage <contract> <slot> --rpc-url https://eth-mainnet...
# → 0x000...124f8  (75000)

# Sapphire: ciphertext
cast storage <contract> <slot> --rpc-url https://testnet.sapphire.oasis.io
# → 0xE7A3B9...  (unreadable)
```

---

## Phase 2: zkTLS via TLSNotary

Phase 2 removes the trust assumption on the Notary. In Phase 1 you trust it to sign accurate data. In Phase 2 it's mathematically unable to sign fabricated data.

**The contract doesn't change.** It still verifies the same ECDSA signature. The Notary's input just goes from "I fetched this from an API" to "here's a cryptographic proof of a real TLS session."

### Step 1 — Client-side TLS session

The user's browser, running a TLSNotary extension, opens a direct TLS connection with the target payroll portal (e.g., ADP, Gusto). The Rust Notary server is **not** in the middle of this connection.

```
User Browser ── TLS 1.3 ──► Payroll Portal
     ↑
TLSNotary extension runs locally
```

### Step 2 — MPC handshake: Notary as blind co-signer

TLSNotary splits the TLS session keys between the user and the Notary via MPC. The Notary verifies the TLS certificate (confirming the data came from `adp.com`) but **cannot see the plaintext response**.

```
User Browser ──── shared TLS key (MPC) ────► adp.com
    │                     │
    └── MPC sub-protocol ──► Rust Notary
                              (checks cert, blind to content)
```

### Step 3 — Selective disclosure

TLSNotary generates a **redacted transcript proof** locally in the browser. The user picks which fields to reveal (gross salary) and which to redact (passwords, account numbers). The resulting proof says:

> "This is a verified TLS response from adp.com. I'm hiding everything except: gross_salary = $75,000."

The Notary can verify the redaction is valid without seeing the hidden fields.

### Step 4 — Same ECDSA output

After verifying the redacted proof, the Notary signs the salary with its secp256k1 key — identical output to Phase 1. `PrivatePayroll.sol` doesn't know or care whether the data came from a simulated API or a zkTLS proof.

```
Phase 1:  Simulated data → Notary signs → ecrecover → encrypted state
Phase 2:  zkTLS proof    → Notary signs → ecrecover → encrypted state
                           ↑ same pipeline ↑
```

### Trust model: Phase 1 vs Phase 2

| Property | Phase 1 | Phase 2 |
|---|---|---|
| Data origin | Trusted on Notary's word | Proven via TLS certificate |
| Notary can lie | Yes | No — MPC prevents it |
| User exposes credentials | N/A (data simulated) | Never — browser only |
| Contract changes required | — | None |

---

## Deployed contracts

| Network | Address |
|---|---|
| Sapphire Testnet | `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024` |
| Trusted Notary | `0xFCAd0B19bB29D4674531d6f115237E16AfCE377c` |

---

## Security notes (Phase 1)

| Threat | Status |
|---|---|
| Malicious Notary | Trust assumption — removed in Phase 2 |
| Replay attacks | Timestamp checked in contract |
| State exposure | Sapphire TEE encryption |
| Signature forgery | secp256k1 ECDSA |
| Other users reading your data | `msg.sender` check in `getMySalary()` |

---

*February 2026 | Oasis ROSE Bloom Grant | Phase 1 live on Sapphire Testnet*
