const hre = require("hardhat");
require("dotenv").config();

/**
 * Test script to verify a salary proof on-chain
 * This simulates the complete flow: Generate proof → Submit to contract
 * 
 * Usage: npx hardhat run scripts/test-verify.js --network sapphire_testnet
 */
async function main() {
    console.log("\n🧪 Testing PrivatePayroll Contract on Sapphire Testnet...\n");

    const [signer] = await hre.ethers.getSigners();
    const employeeAddress = await signer.getAddress();
    const salary = "75000";
    const timestamp = Math.floor(Date.now() / 1000);

    console.log("📋 Test Configuration:");
    console.log("├─ Employee:", employeeAddress);
    console.log("├─ Salary:", salary);
    console.log("├─ Timestamp:", timestamp);
    console.log("└─ Contract:", process.env.VITE_PRIVATE_PAYROLL_ADDRESS || "Not set");
    console.log("");

    // Step 1: Generate valid signature
    console.log("🔐 Step 1: Generating valid signature...");
    const messageHash = hre.ethers.solidityPackedKeccak256(
        ["address", "uint256", "uint256"],
        [employeeAddress, salary, timestamp]
    );
    const signature = await signer.signMessage(hre.ethers.getBytes(messageHash));
    console.log("✅ Signature generated:", signature.slice(0, 20) + "...");
    console.log("");

    // Step 2: Get contract instance
    console.log("📦 Step 2: Connecting to contract...");
    const contractAddress = process.env.VITE_PRIVATE_PAYROLL_ADDRESS || "0xD7dd18b793B263F3adE4B080707F10965b737421";
    const PrivatePayroll = await hre.ethers.getContractAt("PrivatePayroll", contractAddress);
    console.log("✅ Connected to:", contractAddress);
    console.log("");

    // Step 3: Verify notary address
    console.log("🔍 Step 3: Verifying notary configuration...");
    const trustedNotary = await PrivatePayroll.TRUSTED_NOTARY();
    console.log("├─ Contract Notary:", trustedNotary);
    console.log("├─ Current Signer:", employeeAddress);
    console.log("└─ Match:", trustedNotary === employeeAddress ? "✅ Yes" : "❌ No");
    console.log("");

    if (trustedNotary !== employeeAddress) {
        console.log("⚠️  WARNING: Signer is not the trusted notary!");
        console.log("This transaction will fail with 'Invalid Notary Signature'");
        console.log("");
    }

    // Step 4: Submit to blockchain
    console.log("📤 Step 4: Submitting salary verification...");
    try {
        const tx = await PrivatePayroll.verifyAndStoreSalary(salary, timestamp, signature);
        console.log("⏳ Transaction sent:", tx.hash);
        console.log("⏳ Waiting for confirmation...");

        const receipt = await tx.wait();
        console.log("✅ Transaction confirmed!");
        console.log("├─ Block:", receipt.blockNumber);
        console.log("├─ Gas Used:", receipt.gasUsed.toString());
        console.log("└─ Status:", receipt.status === 1 ? "Success" : "Failed");
        console.log("");

        // Step 5: Read back the salary
        console.log("🔍 Step 5: Reading encrypted salary...");
        const storedSalary = await PrivatePayroll.getMySalary();
        console.log("✅ Stored Salary:", storedSalary.toString());
        console.log("└─ Match:", storedSalary.toString() === salary ? "✅ Correct" : "❌ Mismatch");
        console.log("");

        console.log("🎉 Test Completed Successfully!");
        console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        console.log("📍 Explorer:", `https://testnet.explorer.sapphire.oasis.io/tx/${tx.hash}`);
        console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    } catch (error) {
        console.error("❌ Transaction Failed:");
        console.error(error.message);
        if (error.data) {
            console.error("Error Data:", error.data);
        }
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("\n❌ Test Failed:");
        console.error(error);
        process.exit(1);
    });
