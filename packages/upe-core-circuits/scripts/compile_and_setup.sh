#!/bin/bash
set -e

# ==============================================================================
# Universal Privacy Engine (UPE) - Real ZK Core Circuits Build & Setup
# Target Circuit: rwa_shield.circom
# ==============================================================================

CIRCUIT_NAME="rwa_shield"
CIRCUITS_DIR="circuits"
BUILD_DIR="build"

echo "=========================================================="
echo " UPE Core: Building $CIRCUIT_NAME with real PTAU"
echo "=========================================================="

mkdir -p "$BUILD_DIR"

# 1. Compile Circuit to WebAssembly & R1CS
echo "> [1/3] Compiling circuit constraint system..."
circom "$CIRCUITS_DIR/$CIRCUIT_NAME.circom" --r1cs --wasm --sym -l node_modules -o "$BUILD_DIR"
echo "✅ Compilation complete. R1CS and WASM generated in $BUILD_DIR/"

# 2. Trusted Setup (using Hermez PTAU)
echo "> [2/3] Downloading PTAU and running Groth16 setup..."
PTAU_FILE="$BUILD_DIR/powersOfTau28_hez_final_14.ptau"

if [ ! -f "$PTAU_FILE" ]; then
    echo "  -> Downloading powersOfTau28_hez_final_14.ptau..."
    curl -L https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_14.ptau -o "$PTAU_FILE"
fi

ZKEY_FINAL="$BUILD_DIR/${CIRCUIT_NAME}_final.zkey"
echo "  -> Generating zkey (Phase 2)..."
snarkjs groth16 setup "$BUILD_DIR/${CIRCUIT_NAME}.r1cs" "$PTAU_FILE" "$BUILD_DIR/${CIRCUIT_NAME}_0000.zkey"
snarkjs zkey contribute "$BUILD_DIR/${CIRCUIT_NAME}_0000.zkey" "$ZKEY_FINAL" --name="UPE Phase 2 Test" -v -e="$(head -c 32 /dev/urandom | base64)"

echo "✅ Trusted Setup complete. Final ZKey: $ZKEY_FINAL"

# 3. Export Verification Artifacts
echo "> [3/3] Exporting verification key and Solidity verifier..."
VKEY_FILE="$BUILD_DIR/verification_key.json"
VERIFIER_SOL="../../adapters/evm-contracts/src/Verifier.sol"

snarkjs zkey export verificationkey "$ZKEY_FINAL" "$VKEY_FILE"
snarkjs zkey export solidityverifier "$ZKEY_FINAL" "$VERIFIER_SOL"

# Fix solidity pragma to match RWAOracle
sed -i 's/pragma solidity \^0.6.11;/pragma solidity \^0.8.19;/g' "$VERIFIER_SOL" || true

echo "✅ Export complete."
echo ""
echo "=========================================================="
echo " Build successful! 🎉"
echo " Verifier Contract : $VERIFIER_SOL"
echo " VKey JSON         : $VKEY_FILE"
echo "=========================================================="
