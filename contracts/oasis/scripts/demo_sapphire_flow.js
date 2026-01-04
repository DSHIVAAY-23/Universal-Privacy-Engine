const { ethers } = require("hardhat");

/**
 * Oasis Sapphire End-to-End Demo Flow
 * 
 * Actors:
 * - Notary (Deployer): Trusted entity that signs off-chain data.
 * - User (Employee): The beneficiary who submits the proof and owns the private data.
 */

async function main() {
    console.log("\n🌸 Oasis Sapphire Demo: Notary -> Sapphire Flow\n");

    // ============================================================
    // Setup: Actors
    // ============================================================
    const [deployer, employee] = await ethers.getSigners();
    const notary = deployer;
    const user = employee;

    console.log("   Notary (Deployer):", notary.address);
    console.log("   User (Employee):  ", user.address);

    // ============================================================
    // Setup: Deploy PrivatePayroll
    // ============================================================
    console.log("\n📋 Setup: Deploying PrivatePayroll contract...");
    const PrivatePayroll = await ethers.getContractFactory("PrivatePayroll");
    // Pass notary address to constructor
    const contract = await PrivatePayroll.deploy(notary.address);
    await contract.waitForDeployment();
    const contractAddress = await contract.getAddress();
    console.log("   ✅ Contract deployed at:", contractAddress);

    // ============================================================
    // Step 1: Off-Chain STLOP Proof Generation
    // ============================================================
    console.log("\n📋 Step 1: Off-Chain STLOP Proof Generation...");

    const salary = 7500;
    // Use a fixed timestamp or Date.now()
    const timestamp = Math.floor(Date.now() / 1000);

    console.log(`   Data Payload: { salary: ${salary}, timestamp: ${timestamp} }`);

    // Hash: keccak256(abi.encodePacked(user.address, salary, timestamp))
    // Matches Solidity: keccak256(abi.encodePacked(msg.sender, salary, timestamp))
    const messageHash = ethers.solidityPackedKeccak256(
        ["address", "uint256", "uint256"],
        [user.address, salary, timestamp]
    );

    console.log("   Message Hash:", messageHash);

    // Sign: notary.signMessage(binaryHash)
    // ethers.signMessage automatically adds the "\x19Ethereum Signed Message:\n32" prefix
    // This is compatible with Solidity's ecrecover when using the specific prefix construction.
    const signature = await notary.signMessage(ethers.getBytes(messageHash));
    console.log("   Signature:", signature);

    // ============================================================
    // Step 2: On-Chain Verification & Storage
    // ============================================================
    console.log("\n📋 Step 2: On-Chain Verification & Storage...");

    // User submits the proof
    const tx = await contract.connect(user).verifyAndStoreSalary(
        salary,
        timestamp,
        signature
    );

    const receipt = await tx.wait();
    console.log("   ✅ Transaction confirmed in block:", receipt.blockNumber);

    // ============================================================
    // Step 3: Verification (Decrypted View)
    // ============================================================
    console.log("\n📋 Step 3: Verification (Decrypted View)...");

    // User calls getMySalary()
    // On Sapphire, this view call is encrypted, and only the user can see the result.
    const storedSalary = await contract.connect(user).getMySalary();

    if (storedSalary.toString() === salary.toString()) {
        console.log(`\n   ✅ [Sapphire] Private State Decrypted: ${storedSalary} USDC`);
    } else {
        console.error("   ❌ Verification Failed: Salary mismatch.");
        process.exit(1);
    }

    // ============================================================
    // Summary
    // ============================================================
    console.log("\n" + "=".repeat(60));
    console.log("🎉 Demo Complete!");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
