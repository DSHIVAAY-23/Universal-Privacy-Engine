import { ethers } from "hardhat";
import { exec } from "child_process";
import { promisify } from "util";
import path from "path";

const execAsync = promisify(exec);

async function main() {
    const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS || "0x600f0116753576D101f047AbA13dDCa6727f6E40";
    const [signer] = await ethers.getSigners();

    console.log("=================================================");
    console.log("   UPE MVP: Live On-Chain Verification Script");
    console.log("=================================================");
    console.log("Using Signer Account:   ", signer.address);
    console.log("Connecting to RWAOracle:", CONTRACT_ADDRESS);

    const RWAOracle = await ethers.getContractAt("RWAOracle", CONTRACT_ADDRESS);

    console.log("\n[1/4] Generating Real ZK Proof via SnarkJS...");

    const assetContract = "0x0000000000000000000000000000000000000000";
    const minRequiredValue = "500000";
    const scriptPath = path.resolve(__dirname, "../../packages/upe-core-circuits/scripts/generate_proof.js");

    const t0 = performance.now();
    const { stdout, stderr } = await execAsync(`node ${scriptPath} ${minRequiredValue} ${assetContract}`);
    const t1 = performance.now();

    if (stderr && !stderr.includes("Warning") && !stderr.includes("ExperimentalWarning")) {
        console.error("Prover Script Error:", stderr);
        process.exit(1);
    }

    const proofData = JSON.parse(stdout);

    console.log(`✅ Proof generated successfully in ${(t1 - t0).toFixed(2)}ms`);
    console.log("   Generated Nullifier:", proofData.nullifierHash);

    console.log("\n[2/4] Formatting EVM transaction parameters...");
    console.log("   Flat Array Length:", proofData.flatProof.length);

    console.log("\n[3/4] Submitting transaction to live Oasis Sapphire testnet...");

    const tx = await RWAOracle.submitRWAProof(
        proofData.flatProof,
        proofData.nullifierHash,
        proofData.minRequiredValue,
        assetContract,
        { gasLimit: 3000000 }
    );
    console.log(`⏳ Transaction broadcasted! Hash: ${tx.hash}`);

    console.log("\n[4/4] Awaiting execution confirmation on Oasis...");
    const receipt = await tx.wait();

    console.log("\n=================================================");
    console.log(`✅ SUCCESS! Transaction confirmed in block #${receipt.blockNumber}`);
    console.log(`🔗 Explorer URL: https://explorer.oasis.io/testnet/sapphire/tx/${tx.hash}`);
    console.log(`🛡️  On-Chain Verification Gas Used: ${receipt.gasUsed.toString()}`);
    console.log("=================================================\n");
}

main().catch(console.error);
