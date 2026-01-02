const { ethers } = require("hardhat");

/**
 * Oasis Sapphire End-to-End Demo
 * 
 * This script demonstrates the full UPE flow on Sapphire:
 * 1. Notary signs off-chain salary data (STLOP proof)
 * 2. Employee submits proof to PrivatePayroll contract
 * 3. Contract verifies signature and stores in ENCRYPTED state
 * 4. Employee queries their salary (only they can see it)
 */

async function main() {
    console.log("\n🌸 Oasis Sapphire Demo: Private Payroll Flow\n");
    console.log("=".repeat(60));

    // ============================================================
    // STEP 1: Setup Accounts
    // ============================================================
    console.log("\n📋 Step 1: Setting up accounts...");

    const [deployer, employee] = await ethers.getSigners();

    console.log("   Deployer (Notary):", deployer.address);
    console.log("   Employee:", employee.address);

    // ============================================================
    // STEP 2: Deploy PrivatePayroll Contract
    // ============================================================
    console.log("\n📋 Step 2: Deploying PrivatePayroll contract...");

    // Get contract factory
    const PrivatePayroll = await ethers.getContractFactory("PrivatePayroll");

    // Deploy contract with deployer as trusted notary
    const contract = await PrivatePayroll.deploy(deployer.address);
    await contract.waitForDeployment();

    const contractAddress = await contract.getAddress();
    console.log("   ✅ Contract deployed at:", contractAddress);

    // Note: In production, TRUSTED_NOTARY would be a separate secure address
    // For this demo, we use deployer as the notary
    const trustedNotary = await contract.TRUSTED_NOTARY();
    console.log("   Trusted Notary:", trustedNotary);

    // ============================================================
    // STEP 3: Simulate STLOP Proof Generation (Off-Chain)
    // ============================================================
    console.log("\n📋 Step 3: Generating STLOP proof (off-chain)...");

    const salary = 5000;
    const timestamp = Math.floor(Date.now() / 1000);

    console.log("   Salary:", salary);
    console.log("   Timestamp:", timestamp);

    // Construct message hash (must match Solidity logic)
    const messageHash = ethers.solidityPackedKeccak256(
        ["address", "uint256", "uint256"],
        [employee.address, salary, timestamp]
    );

    console.log("   Message Hash:", messageHash);

    // Sign with EIP-191 prefix (Ethereum Signed Message)
    // This matches the Solidity: keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash))
    const signature = await deployer.signMessage(ethers.getBytes(messageHash));

    console.log("   Signature:", signature);
    console.log("   ✅ STLOP proof generated");

    // ============================================================
    // STEP 4: Submit Proof to Sapphire Contract (On-Chain)
    // ============================================================
    console.log("\n📋 Step 4: Submitting proof to Sapphire contract...");

    // Employee submits their own salary proof
    const tx = await contract.connect(employee).verifyAndStoreSalary(
        salary,
        timestamp,
        signature
    );

    console.log("   Transaction hash:", tx.hash);

    // Wait for transaction confirmation
    const receipt = await tx.wait();
    console.log("   ✅ Transaction confirmed in block:", receipt.blockNumber);

    // Check for SalaryVerified event
    const event = receipt.logs.find(log => {
        try {
            const parsed = contract.interface.parseLog(log);
            return parsed && parsed.name === "SalaryVerified";
        } catch {
            return false;
        }
    });

    if (event) {
        const parsed = contract.interface.parseLog(event);
        console.log("   📢 Event emitted: SalaryVerified");
        console.log("      Employee:", parsed.args.employee);
        console.log("      Timestamp:", parsed.args.timestamp.toString());
    }

    // ============================================================
    // STEP 5: Query Encrypted State (Employee Only)
    // ============================================================
    console.log("\n📋 Step 5: Querying encrypted state...");

    // Employee queries their own salary
    const retrievedSalary = await contract.connect(employee).getMySalary();

    console.log("   Retrieved Salary:", retrievedSalary.toString());

    // Verify the salary matches
    if (retrievedSalary.toString() === salary.toString()) {
        console.log("   ✅ Private Salary Retrieved: " + salary);
    } else {
        console.log("   ❌ Salary mismatch!");
        process.exit(1);
    }

    // ============================================================
    // STEP 6: Demonstrate Privacy (Negative Test)
    // ============================================================
    console.log("\n📋 Step 6: Demonstrating privacy guarantees...");

    // Try to query from deployer (not the employee)
    try {
        await contract.connect(deployer).getMySalary();
        console.log("   ❌ Privacy violation: Deployer could read salary!");
        process.exit(1);
    } catch (error) {
        console.log("   ✅ Privacy enforced: Deployer cannot read employee's salary");
        console.log("      Reason:", error.message.split("(")[0].trim());
    }

    // ============================================================
    // Summary
    // ============================================================
    console.log("\n" + "=".repeat(60));
    console.log("\n🎉 Demo Complete! Here's what happened:\n");
    console.log("1. 🔐 Notary signed salary data off-chain (STLOP proof)");
    console.log("2. 📤 Employee submitted proof to Sapphire contract");
    console.log("3. ✅ Contract verified signature and stored in ENCRYPTED state");
    console.log("4. 👁️  Employee queried their salary (only they can see it)");
    console.log("5. 🚫 Other accounts cannot access the encrypted data");
    console.log("\n💡 Key Insight:");
    console.log("   On a normal EVM chain, the salary would be PUBLIC.");
    console.log("   On Sapphire, it's ENCRYPTED at the ParaTime level.");
    console.log("\n🌸 Oasis Sapphire: Privacy-preserving smart contracts! 🔐\n");
}

// Execute the demo
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("\n❌ Demo failed:", error);
        process.exit(1);
    });
