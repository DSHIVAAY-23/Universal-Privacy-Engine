import { useEffect } from 'react';
import { useAccount } from 'wagmi';
import { motion } from 'framer-motion';
import { Card } from '../ui/Card';
import { Badge } from '../ui/Badge';
import { Spinner } from '../ui/Spinner';
import { useGetSalary } from '../../hooks/usePrivatePayroll';
import { useAppStore } from '../../store/useAppStore';

export function SalaryDisplay() {
    const { address, isConnected } = useAccount();
    const { salary, isLoading, error, refetch } = useGetSalary(address);
    const { setSalary } = useAppStore();

    useEffect(() => {
        if (salary) {
            setSalary(salary);
        }
    }, [salary, setSalary]);

    const formatSalary = (amount: bigint) => {
        return new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
            minimumFractionDigits: 0,
        }).format(Number(amount));
    };

    if (!isConnected) {
        return null;
    }

    if (isLoading) {
        return (
            <Card className="max-w-2xl mx-auto text-center py-8">
                <Spinner size="md" />
                <p className="text-gray-400 mt-4">Loading your encrypted salary...</p>
            </Card>
        );
    }

    if (error) {
        return (
            <Card className="max-w-2xl mx-auto">
                <div className="text-center py-8">
                    <div className="inline-flex items-center justify-center w-16 h-16 bg-gray-800/50 rounded-full mb-4">
                        <svg className="w-8 h-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                        </svg>
                    </div>
                    <h3 className="text-lg font-semibold text-white mb-2">No Salary Record Found</h3>
                    <p className="text-gray-400 text-sm mb-4">
                        You haven't verified your salary yet. Click "Verify Income" above to get started.
                    </p>
                    <Badge variant="info">Not Yet Verified</Badge>
                </div>
            </Card>
        );
    }

    if (!salary || salary === 0n) {
        return null;
    }

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
        >
            <Card variant="gradient" className="max-w-2xl mx-auto">
                <div className="text-center mb-6">
                    <div className="inline-flex items-center justify-center w-16 h-16 bg-green-500/10 rounded-full mb-4">
                        <svg className="w-8 h-8 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                        </svg>
                    </div>
                    <h2 className="text-2xl font-bold text-white mb-2">🔒 Your Private Salary</h2>
                    <p className="text-gray-400">Encrypted on Oasis Sapphire</p>
                </div>

                {/* Salary Amount */}
                <div className="bg-gradient-to-br from-gray-800/80 to-gray-900/80 rounded-xl p-8 border border-gray-700 mb-6">
                    <p className="text-sm text-gray-400 mb-2">Verified Annual Salary</p>
                    <p className="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-sapphire-400 to-primary-400">
                        {formatSalary(salary)}
                    </p>
                </div>

                {/* Status Info */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                    <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                        <p className="text-sm text-gray-400 mb-1">Status</p>
                        <Badge variant="success">✅ Verified on Sapphire</Badge>
                    </div>
                    <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                        <p className="text-sm text-gray-400 mb-1">Last Updated</p>
                        <p className="text-sm text-white">{new Date().toLocaleDateString()}</p>
                    </div>
                </div>

                {/* Privacy Info */}
                <div className="p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
                    <div className="flex items-start space-x-3">
                        <svg className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                        </svg>
                        <div>
                            <p className="text-sm font-semibold text-green-400 mb-1">Privacy Guarantee</p>
                            <p className="text-xs text-green-400/80">
                                This value is encrypted using Oasis Sapphire's TEE technology. Only your wallet address can decrypt and view this salary. Even blockchain validators cannot read this data.
                            </p>
                        </div>
                    </div>
                </div>

                {/* Refresh Button */}
                <div className="mt-6 text-center">
                    <button
                        onClick={() => refetch()}
                        className="text-sm text-gray-400 hover:text-sapphire-400 transition-colors inline-flex items-center"
                    >
                        <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                        </svg>
                        Refresh Data
                    </button>
                </div>
            </Card>
        </motion.div>
    );
}
