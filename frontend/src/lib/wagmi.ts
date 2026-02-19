import { http, createConfig } from 'wagmi';
import { sapphireTestnet } from './chains';
import { connectorsForWallets } from '@rainbow-me/rainbowkit';
import {
    metaMaskWallet,
    walletConnectWallet,
    coinbaseWallet,
    rainbowWallet,
} from '@rainbow-me/rainbowkit/wallets';

/**
 * RainbowKit Wallet Connectors Configuration
 */
const connectors = connectorsForWallets(
    [
        {
            groupName: 'Recommended',
            wallets: [metaMaskWallet, rainbowWallet, walletConnectWallet, coinbaseWallet],
        },
    ],
    {
        appName: 'Universal Privacy Engine',
        projectId: import.meta.env.VITE_WALLETCONNECT_PROJECT_ID || 'YOUR_PROJECT_ID',
    }
);

/**
 * Wagmi Configuration for Oasis Sapphire Testnet
 */
export const config = createConfig({
    chains: [sapphireTestnet],
    connectors,
    transports: {
        [sapphireTestnet.id]: http(),
    },
    ssr: false,
});

declare module 'wagmi' {
    interface Register {
        config: typeof config;
    }
}
