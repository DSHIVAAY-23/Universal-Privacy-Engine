require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    solidity: {
        version: "0.8.19",
        settings: {
            optimizer: {
                enabled: true,
                runs: 200
            }
        }
    },
    paths: {
        sources: "./src",
        tests: "./test",
        cache: "./cache",
        artifacts: "./artifacts"
    },
    networks: {
        sapphire_testnet: {
            url: process.env.SAPPHIRE_RPC_URL || "https://testnet.sapphire.oasis.io",
            chainId: 23295, // 0x5aff
            accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
            gasPrice: 100000000000, // 100 gwei
        },
        sapphire_mainnet: {
            url: "https://sapphire.oasis.io",
            chainId: 23294, // 0x5afe
            accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
        }
    },
    etherscan: {
        apiKey: {
            sapphire_testnet: "NOT_REQUIRED",
            sapphire_mainnet: "NOT_REQUIRED"
        },
        customChains: [
            {
                network: "sapphire_testnet",
                chainId: 23295,
                urls: {
                    apiURL: "https://testnet.explorer.sapphire.oasis.io/api",
                    browserURL: "https://testnet.explorer.sapphire.oasis.io"
                }
            },
            {
                network: "sapphire_mainnet",
                chainId: 23294,
                urls: {
                    apiURL: "https://explorer.sapphire.oasis.io/api",
                    browserURL: "https://explorer.sapphire.oasis.io"
                }
            }
        ]
    }
};
