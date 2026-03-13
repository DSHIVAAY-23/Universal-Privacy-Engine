import { ethers } from "hardhat";

async function main() {
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with the account:", deployer.address);

    // Deploying RWAOracle
    // Replace this with the actual ATTESTER address or generate one for the demo
    const attesterAddress = deployer.address;

    const RWAOracle = await ethers.getContractFactory("RWAOracle");
    const rwaOracle = await RWAOracle.deploy(attesterAddress);

    await rwaOracle.waitForDeployment();

    console.log("🚀 RWAOracle deployed to Oasis Sapphire Testnet at:", await rwaOracle.getAddress());
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
