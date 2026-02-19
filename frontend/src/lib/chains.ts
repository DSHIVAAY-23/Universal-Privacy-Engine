import { defineChain } from 'viem';

/**
 * Oasis Sapphire Testnet Chain Definition
 * Official RPC: https://testnet.sapphire.oasis.io
 */
export const sapphireTestnet = defineChain({
    id: 0x5aff, // 23295 in decimal
    name: 'Oasis Sapphire Testnet',
    network: 'sapphire-testnet',
    nativeCurrency: {
        decimals: 18,
        name: 'TEST',
        symbol: 'TEST',
    },
    rpcUrls: {
        default: {
            http: ['https://testnet.sapphire.oasis.io'],
            webSocket: ['wss://testnet.sapphire.oasis.io/ws'],
        },
        public: {
            http: ['https://testnet.sapphire.oasis.io'],
        },
    },
    blockExplorers: {
        default: {
            name: 'Oasis Sapphire Explorer',
            url: 'https://testnet.explorer.sapphire.oasis.io',
        },
    },
    testnet: true,
});
