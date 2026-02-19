import { motion } from 'framer-motion';
import { Card } from '../ui/Card';
import { Badge } from '../ui/Badge';

interface TransactionStatusProps {
    status: 'pending' | 'success' | 'error';
    txHash?: string;
    error?: string;
}

export function TransactionStatus({ status, txHash, error }: TransactionStatusProps) {
    const getExplorerUrl = (hash: string) => {
        return `https://testnet.explorer.sapphire.oasis.io/tx/${hash}`;
    };

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
        >
            <Card className="max-w-2xl mx-auto">
                {status === 'pending' && (
                    <div className="text-center py-8">
                        <div className="inline-flex items-center justify-center w-16 h-16 bg-yellow-500/10 rounded-full mb-4">
                            <svg className="animate-spin h-8 w-8 text-yellow-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                        </div>
                        <h3 className="text-xl font-bold text-white mb-2">⏳ Transaction Pending</h3>
                        <p className="text-gray-400">Waiting for blockchain confirmation...</p>
                        <Badge variant="warning" className="mt-4">Confirming on Sapphire Testnet</Badge>
                    </div>
                )}

                {status === 'success' && txHash && (
                    <div className="text-center py-8">
                        <motion.div
                            initial={{ scale: 0 }}
                            animate={{ scale: 1 }}
                            transition={{ type: 'spring', stiffness: 200, damping: 15 }}
                            className="inline-flex items-center justify-center w-16 h-16 bg-green-500/10 rounded-full mb-4"
                        >
                            <svg className="w-8 h-8 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                            </svg>
                        </motion.div>
                        <h3 className="text-xl font-bold text-white mb-2">✅ Income Verified Successfully!</h3>
                        <p className="text-gray-400 mb-4">Your salary has been encrypted and stored on Oasis Sapphire</p>
                        <Badge variant="success" className="mb-4">Transaction Confirmed</Badge>

                        <div className="mt-6 p-4 bg-gray-800/50 rounded-lg border border-gray-700">
                            <p className="text-sm text-gray-400 mb-2">Transaction Hash</p>
                            <p className="text-xs font-mono text-gray-300 break-all mb-3">{txHash}</p>
                            <a
                                href={getExplorerUrl(txHash)}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="inline-flex items-center text-sm text-sapphire-400 hover:text-sapphire-300 transition-colors"
                            >
                                View on Explorer
                                <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                                </svg>
                            </a>
                        </div>
                    </div>
                )}

                {status === 'error' && (
                    <div className="text-center py-8">
                        <div className="inline-flex items-center justify-center w-16 h-16 bg-red-500/10 rounded-full mb-4">
                            <svg className="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </div>
                        <h3 className="text-xl font-bold text-white mb-2">❌ Transaction Failed</h3>
                        <p className="text-gray-400 mb-4">There was an error submitting your transaction</p>
                        <Badge variant="error" className="mb-4">Error</Badge>

                        {error && (
                            <div className="mt-6 p-4 bg-red-500/10 border border-red-500/20 rounded-lg">
                                <p className="text-sm text-red-400">{error}</p>
                            </div>
                        )}
                    </div>
                )}
            </Card>
        </motion.div>
    );
}
