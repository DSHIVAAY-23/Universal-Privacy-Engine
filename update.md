# 📋 UPE Update Log — `feat/multi-chain-rwa`
*Last updated: 2026-03-26*

---

## What Was Done This Session

### 1. ✅ Secret Network Build — Fixed
**File:** `adapters/secret-network-cosmwasm/`

- **Problem:** `Cargo.toml` had `ark-bn254`, `ark-groth16`, `ark-snark`, `ark-serialize` as deps — all incompatible with CosmWasm WASM compilation target, causing 2 hard errors and 7 warnings.
- **Fix:** Removed all `ark_*` crates. Replaced with `base64`, `hex`, `sha2`.
- **New logic in `src/lib.rs`:**
  - Attester-gated proof submission (only the trusted UPE Notary can submit proofs until full on-chain Groth16 is available via `ark-cosmwasm`)
  - Structural base64 proof validation (must be exactly 256 bytes = valid Groth16/BN254 serialization)
  - Clear error messages explaining Phase 2 roadmap for trustless verification
  - `GetAttester` query endpoint added
- **Result:** `cargo check` exits 0, zero errors, zero warnings.

---

### 2. ✅ zkSync Era — Contracts Compiled
**File:** `adapters/zksync-solidity/`

- Ran `npx hardhat compile` — all 3 Solidity files compiled successfully with `zksolc v1.5.0 + zkvm-solc v0.8.20`
- Added `.env.example` with `DEPLOYER_PRIVATE_KEY` and `ZKSYNC_RPC_URL` placeholders
- **Deploy command ready:** `npm run deploy:testnet` (needs funded wallet)

---

### 3. ✅ `deploy-all.sh` Created
**File:** `deploy-all.sh` (repo root, executable)

Single command to deploy all chains:
```bash
./deploy-all.sh 0x<your-private-key>
```
- Auto-deploys zkSync Era Sepolia via hardhat
- Builds Secret Network WASM + prints `secretcli` instantiation commands
- Prints Mina (`zk deploy`) instructions
- Summarizes all addresses at the end

---

### 4. ✅ `contracts.ts` — Both Files Updated

| File | What Changed |
|---|---|
| `apps/web/lib/contracts.ts` | Oasis address set to live `0x868d...`, others env-var driven with `⏳` comments |
| `frontend/src/lib/contracts.ts` | Same + added `NETWORK_INFO` map (label, icon, chainId, isEVM) for each network |

**Oasis Sapphire (live):** `0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA`

---

### 5. ✅ `TESTNET_LAUNCH_CHECKLIST.md` — Rewritten
Step-by-step checklist for all 5 chains with faucet links, commands, and env var tracking.

---

### 6. ✅ `apps/web` Build — Verified
`npx next build` → **Exit code 0** ✅
`npm run dev` → **Compiled & running** on `http://localhost:3000` ✅

---

## Current Address Status

| Network | Status | Address |
|---|---|---|
| Oasis Sapphire Testnet | ✅ **Live** | `0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA` |
| zkSync Era Sepolia | ⏳ Ready to deploy | Run `npm run deploy:testnet` (needs wallet ETH) |
| Secret Network Pulsar-3 | ⏳ Ready to deploy | Run `secretcli tx wasm store ...` |
| Mina Berkeley | ⏳ Ready to deploy | Run `zk deploy` in `adapters/mina-o1js/` |
| Aleo Testnet | ⏳ Ready to deploy | Run `leo deploy` |

---

## What's Next

1. **Fund a deployer wallet** with testnet ETH (zkSync Sepolia), SCRT, and MINA tokens
2. **Run `./deploy-all.sh 0x<key>`** → captures zkSync address automatically
3. **Set env vars** in `apps/web/.env.local`:
   ```
   NEXT_PUBLIC_ZKSYNC_ADDRESS=0x...
   NEXT_PUBLIC_SECRET_ADDRESS=secret1...
   NEXT_PUBLIC_MINA_ADDRESS=B62...
   ```
4. **Test full pipeline** — select each network in the UI, submit a ZK proof end-to-end
