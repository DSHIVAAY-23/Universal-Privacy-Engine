import React from 'react';
import ReactDOM from 'react-dom/client';
import { WagmiProvider } from 'wagmi';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RainbowKitProvider, darkTheme } from '@rainbow-me/rainbowkit';
import App from './App';
import { config } from './lib/wagmi';
import './styles/globals.css';
import '@rainbow-me/rainbowkit/styles.css';

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <WagmiProvider config={config}>
            <QueryClientProvider client={queryClient}>
                <RainbowKitProvider
                    theme={darkTheme({
                        accentColor: '#3b82f6',
                        accentColorForeground: 'white',
                        borderRadius: 'large',
                    })}
                >
                    <App />
                </RainbowKitProvider>
            </QueryClientProvider>
        </WagmiProvider>
    </React.StrictMode>
);
