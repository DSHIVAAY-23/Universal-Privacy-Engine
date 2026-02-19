import { ConnectButton } from '@rainbow-me/rainbowkit';
import { motion } from 'framer-motion';

export function Header() {
    return (
        <motion.header
            initial={{ y: -20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            className="border-b border-gray-800 bg-gray-900/50 backdrop-blur-md sticky top-0 z-50"
        >
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex justify-between items-center h-16">
                    {/* Logo */}
                    <div className="flex items-center space-x-3">
                        <div className="w-10 h-10 bg-gradient-to-br from-sapphire-500 to-primary-600 rounded-lg flex items-center justify-center">
                            <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                            </svg>
                        </div>
                        <div>
                            <h1 className="text-xl font-bold text-white">Universal Privacy Engine</h1>
                            <p className="text-xs text-gray-400">Powered by Oasis Sapphire</p>
                        </div>
                    </div>

                    {/* Wallet Connection */}
                    <div className="flex items-center space-x-4">
                        <ConnectButton
                            chainStatus="icon"
                            showBalance={false}
                        />
                    </div>
                </div>
            </div>
        </motion.header>
    );
}
