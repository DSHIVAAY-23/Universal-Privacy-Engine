import { ReactNode } from 'react';
import { motion } from 'framer-motion';

interface CardProps {
    children: ReactNode;
    className?: string;
    variant?: 'default' | 'gradient';
}

export function Card({ children, className = '', variant = 'default' }: CardProps) {
    const baseStyles = 'rounded-xl p-6 backdrop-blur-sm';

    const variants = {
        default: 'bg-gray-900/50 border border-gray-800',
        gradient: 'bg-gradient-to-br from-gray-900/80 to-gray-800/80 border border-gray-700',
    };

    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
            className={`${baseStyles} ${variants[variant]} ${className}`}
        >
            {children}
        </motion.div>
    );
}
