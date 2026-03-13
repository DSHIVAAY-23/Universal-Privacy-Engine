const snarkjs = require("snarkjs");
const { buildPoseidon } = require("circomlibjs");
const path = require("path");

async function main() {
    const minRequiredValue = process.argv[2] || "500000";
    const userAddressValue = process.argv[3] || "0x123abc";

    // Convert hex address to numeric, or just use a dummy numeric if it fails parsing
    let userAddressNum;
    try {
        if (userAddressValue.startsWith("0x")) {
            userAddressNum = BigInt(userAddressValue).toString();
        } else {
            userAddressNum = BigInt("0x" + userAddressValue).toString();
        }
    } catch (e) {
        userAddressNum = "123456789";
    }

    // 1. Circomlibjs Poseidon instance
    const poseidon = await buildPoseidon();
    const F = poseidon.F;

    // 2. Leaf Generation
    // Assume user has minRequiredValue + 500 dollars
    const tokenBalance = (BigInt(minRequiredValue) + 500n).toString();
    const leaf = poseidon([userAddressNum, tokenBalance]);

    // 3. Merkle Proof (Mock a 20-level tree with 0s)
    const nLevels = 20;
    const merklePathElements = Array(nLevels).fill("0");
    const merklePathIndices = Array(nLevels).fill(0);

    let currentHash = leaf;
    for (let i = 0; i < nLevels; i++) {
        // Since index is 0, currentHash is on the left
        const left = currentHash;
        const right = F.e("0");
        currentHash = poseidon([left, right]);
    }
    const stateRoot = F.toObject(currentHash).toString();

    // 4. Nullifier Generation
    const secretTrapdoor = "9876543210123456789"; // Dummy secret
    const nullifierHash = F.toObject(poseidon([userAddressNum, secretTrapdoor])).toString();

    // 5. Build inputs
    const input = {
        stateRoot: stateRoot,
        minRequiredValue: minRequiredValue.toString(),
        nullifierHash: nullifierHash,
        userAddress: userAddressNum,
        tokenBalance: tokenBalance,
        secretTrapdoor: secretTrapdoor,
        merklePathElements: merklePathElements,
        merklePathIndices: merklePathIndices
    };

    // 6. Generate real proof
    const wasmPath = path.join(__dirname, "../build/rwa_shield_js/rwa_shield.wasm");
    const zkeyPath = path.join(__dirname, "../build/rwa_shield_final.zkey");

    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);

    // Flat proof matching uint256[8] for smart contract:
    // [ proof.pi_a[0], proof.pi_a[1], proof.pi_b[0][1], proof.pi_b[0][0], proof.pi_b[1][1], proof.pi_b[1][0], proof.pi_c[0], proof.pi_c[1] ]
    const pA = [proof.pi_a[0], proof.pi_a[1]];
    // Note: Solidity verifies pi_b reversed in snarkjs format!
    const pB = [
        [proof.pi_b[0][1], proof.pi_b[0][0]],
        [proof.pi_b[1][1], proof.pi_b[1][0]]
    ];
    const pC = [proof.pi_c[0], proof.pi_c[1]];

    const flatProof = [
        pA[0], pA[1],
        pB[0][0], pB[0][1],
        pB[1][0], pB[1][1],
        pC[0], pC[1]
    ];

    const out = {
        proof: proof,
        flatProof: flatProof,
        publicSignals: publicSignals,
        nullifierHash: nullifierHash,
        stateRoot: stateRoot,
        minRequiredValue: minRequiredValue.toString(),
        assetContract: userAddressValue
    };

    console.log(JSON.stringify(out));
    process.exit(0);
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
