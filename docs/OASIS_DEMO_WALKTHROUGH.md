# Demo Walkthrough — UPE on Oasis Sapphire

## Live deployment

| | |
|---|---|
| Frontend | [universal-privacy-engine.vercel.app](https://universal-privacy-engine-a1kfpf0no-dshivaay23s-projects.vercel.app) |
| Contract | `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024` (Sapphire Testnet) |
| Notary | `0xFCAd0B19bB29D4674531d6f115237E16AfCE377c` |
| Explorer | [testnet.explorer.sapphire.oasis.io](https://testnet.explorer.sapphire.oasis.io) |

---

## Browser demo (2 minutes)

You need MetaMask configured for Sapphire Testnet (Chain ID: 23295).

1. Open the frontend
2. Connect MetaMask, switch to Sapphire Testnet
3. Click **Start Verification** — frontend calls the Rust Notary, gets a signed STLOP proof back
4. Review the proof (salary, timestamp, ECDSA signature)
5. Click **Submit** — MetaMask prompts you to sign the transaction
6. Wait for confirmation — salary is now in TEE-encrypted state
7. Click **View My Salary** — `getMySalary()` returns your value; any other wallet gets a revert

Confirmed transaction: `0x9def61f121055ff791ba8780ce1ba6596c5a7a6cce995bb035adaaecc9eb2211`

---

## Local reproduction

```bash
# Start the Notary
cd core && cp .env.example .env
# Set NOTARY_PRIVATE_KEY
PORT=3002 cargo run --release

# Verify it's up
curl -s http://localhost:3002/api/health | jq
# → { "status": "ok", "notary_address": "0xfcad..." }

# Generate a proof
curl -s -X POST http://localhost:3002/api/generate-proof \
  -H "Content-Type: application/json" \
  -d '{"employee_address": "0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7"}' | jq
# → { "salary": "75000", "timestamp": ..., "signature": "0x..." }

# Start the frontend
cd frontend && cp .env.example .env
# Set VITE_NOTARY_API_URL=http://localhost:3002
npm install && npm run dev

# Run tests
cd core && cargo test
cd contracts/oasis && forge test -vvv
```

---

## What this actually proves

| Claim | Verification |
|---|---|
| ECDSA sig verified on-chain | `ecrecover` in `PrivatePayroll.sol` recovers `TRUSTED_NOTARY` |
| Storage is encrypted | `eth_getStorageAt` returns ciphertext, not the salary value |
| Access control works | `getMySalary()` reverts for any address other than the employee |
| Replay attacks blocked | Contract checks timestamp uniqueness |
| End-to-end on Sapphire Testnet | Confirmed tx on block explorer |

---

## The key point about Sapphire

```bash
# Ethereum — "private" is a lie:
cast storage <contract> <slot> --rpc-url https://eth-mainnet.g.alchemy.com/v2/...
# 0x000000000000000000000000000000000000000000000000000000000000124f8
# (that's 75000 — your salary, readable by anyone)

# Sapphire — actually encrypted:
cast storage <contract> <slot> --rpc-url https://testnet.sapphire.oasis.io
# 0xE7A3B9C2...  (ciphertext — unreadable without TEE access)
```

Sapphire is the only EVM where `private` mappings are actually private.

---

*February 2026 | Sapphire Testnet*
