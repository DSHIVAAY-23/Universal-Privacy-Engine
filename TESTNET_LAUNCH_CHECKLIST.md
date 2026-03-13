# 🚀 UPE MVP: Live Oasis Sapphire Testnet Launch Checklist

Follow these exact steps to run the complete end-to-end ZK Prover pipeline natively on the Oasis Sapphire Testnet.

---

### Step 1: Boot the Local Rust ZK Prover API
Open a **new terminal tab** and start the lightweight Rust API that formats our ZK Proofs and Nullifiers.
```bash
cd /data/Universal-Privacy-Engine/packages/upe-core-circuits
cargo run
```
*Wait until you see:* `UPE Rust Prover Node listening on 0.0.0.0:8080`

### Step 2: Start the Next.js Frontend
Open a **second terminal tab** to launch the UI. The environment variables are already configured to target your testnet deployment: `0x600f0116753576D101f047AbA13dDCa6727f6E40`.
```bash
cd /data/Universal-Privacy-Engine
./launch_demo.sh
```

### Step 3: Execute the Live Transaction in Browser
1. Open `http://localhost:3000` in your web browser.
2. Click **"Connect Wallet"**. MetaMask will prompt you to switch to the **Oasis Sapphire Testnet** if you are on any other network.
3. Once connected, your `0x...` address will appear top right.
4. Input a dummy Asset Contract and desired Collateral Value.
5. Click **"Generate & Anchor Nullifier On-Chain"**.

### Step 4: Verify the Complete Pipeline
- **Observe the Prover Console**: Watch it sequentially reach out to the Rust `localhost:8080` backend, fetching the deterministic Groth16 structured mock proof.
- **Sign the Transaction**: MetaMask will pop up requesting you to sign the `submitRWAProof` transaction. Gas limits have been adjusted to ensure smooth processing on the testnet.
- **On-Chain Success**: The terminal will pause on `[NETWORK] Awaiting on-chain confirmation...` until Oasis mines the block.
- Upon mining, a brilliant neon `[SUCCESS]` log alongside the live testnet `tx.hash` will appear!

🎉 **You are now successfully anchoring cryptographic ZK nullifiers to a live network execution environment!**
