import { useAppStore } from '../../store/useAppStore';
import { Network } from '../../types';
import { motion } from 'framer-motion';

const NETWORK_METADATA = {
    [Network.OasisSapphire]: {
        name: 'Oasis Sapphire',
        icon: '💎',
        description: 'Confidential EVM (TEE)',
    },
    [Network.zkSyncEra]: {
        name: 'zkSync Era',
        icon: '⚡',
        description: 'ZK-Rollup (EVM)',
    },
    [Network.SecretNetwork]: {
        name: 'Secret Network',
        icon: '🤫',
        description: 'Privacy Preserving TEE',
    },
    [Network.Aleo]: {
        name: 'Aleo',
        icon: '🏔️',
        description: 'Native ZK (Leo)',
    },
    [Network.Mina]: {
        name: 'Mina Protocol',
        icon: '🪶',
        description: 'Succinct ZK (o1js)',
    },
};

export function NetworkSelector() {
    const { selectedNetwork, setSelectedNetwork } = useAppStore();

    return (
        <div className="grid grid-cols-1 md:grid-cols-5 gap-3 mb-8">
            {(Object.entries(NETWORK_METADATA) as [Network, typeof NETWORK_METADATA[Network]][]).map(([key, meta]) => (
                <motion.button
                    key={key}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => setSelectedNetwork(key)}
                    className={`p-4 rounded-xl border transition-all text-left ${
                        selectedNetwork === key
                            ? 'bg-sapphire-500/20 border-sapphire-500 shadow-[0_0_20px_rgba(59,130,246,0.2)]'
                            : 'bg-gray-800/40 border-gray-700 hover:border-gray-500'
                    }`}
                >
                    <div className="text-2xl mb-2">{meta.icon}</div>
                    <div className="text-sm font-bold text-white">{meta.name}</div>
                    <div className="text-[10px] text-gray-400 uppercase tracking-wider">{meta.description}</div>
                </motion.button>
            ))}
        </div>
    );
}
