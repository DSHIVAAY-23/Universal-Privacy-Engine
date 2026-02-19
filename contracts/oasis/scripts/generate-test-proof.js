const { ethers } = require("ethers");
require("dotenv").config();

/**
 * Generate a properly signed STLOP proof for testing
 * This simulates what the Rust Notary service will do in production
 * 
 * Usage: node scripts/generate-test-proof.js <employee_address> <salary>
 */
async function generateTestProof() {
    const employeeAddress = process.argv[2] || process.env.TEST_EMPLOYEE_ADDRESS;
    const salary = process.argv[3] || "75000";

    if (!employeeAddress) {
        console.error("❌ Error: Employee address required");
        console.log("Usage: node scripts/generate-test-proof.js <employee_address> <salary>");
        console.log("Example: node scripts/generate-test-proof.js 0x06deedD21AfE4ae6BFb443A4f560aD13d81e05a7 75000");
        process.exit(1);
    }

    if (!process.env.PRIVATE_KEY) {
        console.error("❌ Error: PRIVATE_KEY not found in .env");
        process.exit(1);
    }

    // Create wallet (this is the "Notary" for testing)
    const wallet = new ethers.Wallet(process.env.PRIVATE_KEY);
    const timestamp = Math.floor(Date.now() / 1000);

    console.log("\n🔐 Generating Test STLOP Proof...\n");
    console.log("📋 Proof Details:");
    console.log("├─ Employee:", employeeAddress);
    console.log("├─ Salary:", salary);
    console.log("├─ Timestamp:", timestamp, `(${new Date(timestamp * 1000).toISOString()})`);
    console.log("└─ Notary:", wallet.address);
    console.log("");

    // Reconstruct the message hash (same as contract)
    const messageHash = ethers.solidityPackedKeccak256(
        ["address", "uint256", "uint256"],
        [employeeAddress, salary, timestamp]
    );

    console.log("📝 Message Hash:", messageHash);

    // Sign with EIP-191 prefix (same as contract expects)
    const signature = await wallet.signMessage(ethers.getBytes(messageHash));

    console.log("✍️  Signature:", signature);
    console.log("");

    // Output as JSON for easy copy-paste
    const proof = {
        salary: salary,
        timestamp: timestamp,
        signature: signature,
        notary_pubkey: wallet.address
    };

    console.log("📦 STLOP Proof (JSON):");
    console.log(JSON.stringify(proof, null, 2));
    console.log("");

    // Verify the signature locally
    const recoveredAddress = ethers.verifyMessage(ethers.getBytes(messageHash), signature);
    console.log("🔍 Verification:");
    console.log("├─ Recovered Signer:", recoveredAddress);
    console.log("└─ Match:", recoveredAddress === wallet.address ? "✅ Valid" : "❌ Invalid");
    console.log("");

    console.log("📝 Next Steps:");
    console.log("1. Copy the JSON proof above");
    console.log("2. Use it in your frontend or test it directly:");
    console.log(`   npx hardhat run scripts/test-verify.js --network sapphire_testnet`);
    console.log("");

    return proof;
}

generateTestProof()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("❌ Error:", error.message);
        process.exit(1);
    });
