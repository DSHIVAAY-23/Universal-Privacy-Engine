import { HardhatUserConfig } from "hardhat/config";
import "@matterlabs/hardhat-zksync-solc";
import "@matterlabs/hardhat-zksync-deploy";

const config: HardhatUserConfig = {
    zksolc: {
        version: "1.5.0",
        settings: {},
    },
    solidity: {
        version: "0.8.20",
        settings: {
            optimizer: { enabled: true, runs: 200 },
        },
    },
    defaultNetwork: "zkSyncTestnet",
    networks: {
        zkSyncTestnet: {
            url: process.env.ZKSYNC_RPC_URL ?? "https://sepolia.era.zksync.dev",
            ethNetwork: "sepolia",
            zksync: true,
            accounts: process.env.DEPLOYER_PRIVATE_KEY
                ? [process.env.DEPLOYER_PRIVATE_KEY]
                : [],
        },
        zkSyncMainnet: {
            url: "https://mainnet.era.zksync.io",
            ethNetwork: "mainnet",
            zksync: true,
            accounts: process.env.DEPLOYER_PRIVATE_KEY
                ? [process.env.DEPLOYER_PRIVATE_KEY]
                : [],
        },
    },
};

export default config;
