#!/bin/bash
set -e

# ==============================================================================
# Universal Privacy Engine (UPE) - Core Circuits Build & Setup
# Target Circuit: rwa_shield.circom
# ==============================================================================

CIRCUIT_NAME="rwa_shield"
CIRCUITS_DIR="circuits"
BUILD_DIR="build"

echo "=========================================================="
echo " UPE Core: Building $CIRCUIT_NAME"
echo "=========================================================="

mkdir -p "$BUILD_DIR"

# 1. Compile Circuit to WebAssembly & R1CS (Constraint System)
echo "> [1/3] Compiling circuit constraint system..."
if [ ! -f "$CIRCUITS_DIR/$CIRCUIT_NAME.circom" ]; then
    echo "❌ Error: $CIRCUITS_DIR/$CIRCUIT_NAME.circom not found!"
    exit 1
fi

circom "$CIRCUITS_DIR/$CIRCUIT_NAME.circom" --r1cs --wasm --sym -o "$BUILD_DIR"
echo "✅ Compilation complete. R1CS and WASM generated in $BUILD_DIR/"

# 2. Trusted Setup (Mock Local Setup for Dev/Testing)
# NOTE: In production, Phase 1 (PTAU) is downloaded from the Hermez perpetual powers of tau.
echo "> [2/3] Running Groth16 setup (using local PTAU phase 1)..."
PTAU_FILE="$BUILD_DIR/pot15_final.ptau"

if [ ! -f "$PTAU_FILE" ]; then
    echo "  -> Generating mock PTAU (powers of tau) file for testing..."
    snarkjs powersoftau new bn128 15 "$BUILD_DIR/pot15_0000.ptau" -v
    snarkjs powersoftau contribute "$BUILD_DIR/pot15_0000.ptau" "$PTAU_FILE" --name="UPE Local Test" -v -e="$(head -c 32 /dev/urandom | base64)"
fi

ZKEY_FINAL="$BUILD_DIR/${CIRCUIT_NAME}_final.zkey"
echo "  -> Generating zkey (Phase 2)..."
snarkjs groth16 setup "$BUILD_DIR/${CIRCUIT_NAME}.r1cs" "$PTAU_FILE" "$BUILD_DIR/${CIRCUIT_NAME}_0000.zkey"
snarkjs zkey contribute "$BUILD_DIR/${CIRCUIT_NAME}_0000.zkey" "$ZKEY_FINAL" --name="UPE Phase 2 Test" -v -e="$(head -c 32 /dev/urandom | base64)"

echo "✅ Trusted Setup complete. Final ZKey: $ZKEY_FINAL"

# 3. Export Verification Artifacts
echo "> [3/3] Exporting verification key and Solidity verifier..."
VKEY_FILE="$BUILD_DIR/verification_key.json"
VERIFIER_SOL="$BUILD_DIR/verifier.sol"

snarkjs zkey export verificationkey "$ZKEY_FINAL" "$VKEY_FILE"
snarkjs zkey export solidityverifier "$ZKEY_FINAL" "$VERIFIER_SOL"

echo "✅ Export complete."
echo ""
echo "=========================================================="
echo " Build successful! 🎉"
echo " Verifier Contract : $VERIFIER_SOL"
echo " VKey JSON         : $VKEY_FILE"
echo "=========================================================="
