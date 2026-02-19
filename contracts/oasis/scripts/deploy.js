const hre = require("hardhat");
require("dotenv").config();

/**
 * Deployment Script for PrivatePayroll Contract
 * Deploys to Oasis Sapphire Testnet (Confidential EVM)
 * 
 * Usage: npx hardhat run scripts/deploy.js --network sapphire_testnet
 */
async function main() {
    console.log("\n🚀 Starting PrivatePayroll Deployment to Oasis Sapphire Testnet...\n");

    // 1. Validate Environment Variables
    const trustedNotaryAddress = process.env.TRUSTED_NOTARY_ADDRESS;

    if (!trustedNotaryAddress || trustedNotaryAddress === "0x0000000000000000000000000000000000000000") {
        throw new Error(
            "❌ TRUSTED_NOTARY_ADDRESS is not set in .env file!\n" +
            "Please set the public address of your Rust Notary service.\n" +
            "Example: TRUSTED_NOTARY_ADDRESS=0x1234567890123456789012345678901234567890"
        );
    }

    if (!process.env.PRIVATE_KEY) {
        throw new Error(
            "❌ PRIVATE_KEY is not set in .env file!\n" +
            "Please add your deployer wallet's private key.\n" +
            "Get testnet tokens from: https://faucet.testnet.oasis.io/"
        );
    }

    // 2. Get Deployer Account
    const [deployer] = await hre.ethers.getSigners();
    const deployerAddress = await deployer.getAddress();
    const balance = await hre.ethers.provider.getBalance(deployerAddress);

    console.log("📋 Deployment Configuration:");
    console.log("├─ Network:", hre.network.name);
    console.log("├─ Chain ID:", (await hre.ethers.provider.getNetwork()).chainId);
    console.log("├─ Deployer Address:", deployerAddress);
    console.log("├─ Deployer Balance:", hre.ethers.formatEther(balance), "TEST");
    console.log("└─ Trusted Notary:", trustedNotaryAddress);
    console.log("");

    // 3. Check Balance
    if (balance === 0n) {
        throw new Error(
            "❌ Deployer wallet has 0 balance!\n" +
            "Get testnet tokens from: https://faucet.testnet.oasis.io/"
        );
    }

    // 4. Deploy Contract
    console.log("📦 Deploying PrivatePayroll contract...");
    const PrivatePayroll = await hre.ethers.getContractFactory("PrivatePayroll");
    const privatePayroll = await PrivatePayroll.deploy(trustedNotaryAddress);

    console.log("⏳ Waiting for deployment transaction to be mined...");
    await privatePayroll.waitForDeployment();

    const contractAddress = await privatePayroll.getAddress();

    console.log("\n✅ PrivatePayroll Contract Deployed Successfully!");
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    console.log("📍 Contract Address:", contractAddress);
    console.log("🔗 Explorer:", `https://testnet.explorer.sapphire.oasis.io/address/${contractAddress}`);
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 5. Verify Contract Configuration
    console.log("\n🔍 Verifying Contract Configuration...");
    const storedNotary = await privatePayroll.TRUSTED_NOTARY();
    console.log("├─ Stored Notary Address:", storedNotary);
    console.log("└─ Match:", storedNotary === trustedNotaryAddress ? "✅ Correct" : "❌ Mismatch");

    // 6. Update Frontend .env Instructions
    console.log("\n📝 Next Steps:");
    console.log("1. Copy the contract address above");
    console.log("2. Update frontend/.env file:");
    console.log(`   VITE_PRIVATE_PAYROLL_ADDRESS=${contractAddress}`);
    console.log("3. Restart the frontend dev server");
    console.log("4. Test the complete flow: Wallet → Proof → Blockchain → View Salary\n");
}

// Execute deployment
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("\n❌ Deployment Failed:");
        console.error(error);
        process.exit(1);
    });
