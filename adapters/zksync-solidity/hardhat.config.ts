import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
    solidity: {
        version: "0.8.20",
        settings: {
            optimizer: { enabled: true, runs: 200 },
        },
    },
    networks: {
        zkSyncTestnet: {
            url: process.env.ZKSYNC_RPC_URL ?? "https://testnet.era.zksync.dev",
            accounts: process.env.DEPLOYER_PRIVATE_KEY
                ? [process.env.DEPLOYER_PRIVATE_KEY]
                : [],
        },
        zkSyncMainnet: {
            url: "https://mainnet.era.zksync.io",
            accounts: process.env.DEPLOYER_PRIVATE_KEY
                ? [process.env.DEPLOYER_PRIVATE_KEY]
                : [],
        },
    },
};

export default config;
