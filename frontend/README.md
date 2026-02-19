# Universal Privacy Engine - Frontend

Production-ready React/TypeScript frontend for the Universal Privacy Engine MVP, built according to the specifications in `MVP_ARCHITECTURE.md`.

## 🚀 Features

- **Wallet Integration**: RainbowKit with MetaMask, WalletConnect, Coinbase Wallet, and Rainbow Wallet support
- **Oasis Sapphire**: Native integration with Sapphire Testnet for encrypted on-chain state
- **STLOP Proofs**: Notary API integration for cryptographic salary verification
- **Premium UI**: Dark mode, Tailwind CSS, Framer Motion animations
- **Type Safety**: Strict TypeScript with Wagmi and Viem
- **State Management**: Zustand for global application state

## 📋 Prerequisites

- Node.js 18+ and npm
- MetaMask or compatible Web3 wallet
- Access to Oasis Sapphire Testnet

## 🛠️ Installation

```bash
# Install dependencies
npm install

# Copy environment variables
cp .env.example .env

# Update .env with your configuration:
# - VITE_NOTARY_API_URL: Your Rust Notary API endpoint
# - VITE_PRIVATE_PAYROLL_ADDRESS: Deployed PrivatePayroll contract address
# - VITE_WALLETCONNECT_PROJECT_ID: Get from https://cloud.walletconnect.com
```

## 🏃 Running the Application

```bash
# Development server (http://localhost:3000)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## 📁 Project Structure

```
src/
├── components/
│   ├── layout/          # Header, Layout components
│   ├── ui/              # Reusable UI components (Button, Card, Spinner, Badge)
│   └── verify/          # Verification flow components
├── hooks/
│   ├── useNotaryAPI.ts       # Notary API integration
│   └── usePrivatePayroll.ts  # Smart contract interactions
├── lib/
│   ├── chains.ts        # Sapphire Testnet configuration
│   ├── contracts.ts     # Contract ABIs and addresses
│   ├── notary.ts        # Notary API client
│   └── wagmi.ts         # Wagmi configuration
├── store/
│   └── useAppStore.ts   # Zustand global state
├── types/
│   └── index.ts         # TypeScript interfaces
└── styles/
    └── globals.css      # Tailwind CSS and global styles
```

## 🔧 Configuration

### Environment Variables

- `VITE_NOTARY_API_URL`: Rust Notary Service endpoint (default: `http://localhost:3001`)
- `VITE_PRIVATE_PAYROLL_ADDRESS`: PrivatePayroll contract address on Sapphire Testnet
- `VITE_WALLETCONNECT_PROJECT_ID`: WalletConnect Cloud project ID

### Oasis Sapphire Testnet

The application is configured to connect to Oasis Sapphire Testnet:
- **Chain ID**: 0x5aff (23295)
- **RPC URL**: https://testnet.sapphire.oasis.io
- **Explorer**: https://testnet.explorer.sapphire.oasis.io

## 🎯 User Journey

1. **Connect Wallet**: Click "Connect Wallet" in the header
2. **Verify Income**: Click "Start Verification" to generate STLOP proof
3. **Preview Proof**: Review salary, timestamp, and cryptographic signature
4. **Submit to Blockchain**: Approve MetaMask transaction
5. **View Encrypted Salary**: See your verified salary (only you can decrypt it)

## 🔐 Security Features

- **Encrypted State**: All salary data is encrypted by Sapphire's TEE
- **STLOP Proofs**: Cryptographic signatures ensure data authenticity
- **Private Access**: Only the wallet owner can decrypt their salary
- **Type Safety**: Strict TypeScript prevents runtime errors

## 🧪 Development Mode

When the Notary API is unavailable, the application automatically falls back to mock proofs for development:

```typescript
// Mock proof generated with realistic data
{
  salary: "75000",
  timestamp: <current_timestamp>,
  signature: "0x1111...",  // Mock signature
  notary_pubkey: "0x2222..."  // Mock pubkey
}
```

## 📦 Dependencies

### Core
- React 18.2.0
- TypeScript 5.3.3
- Vite 5.1.0

### Web3
- Wagmi 2.5.7
- Viem 2.7.13
- RainbowKit 2.0.2

### UI
- Tailwind CSS 3.4.1
- Framer Motion 11.0.3

### State & Data
- Zustand 4.5.0
- Axios 1.6.7
- TanStack React Query 5.22.2

## 🚢 Deployment

### Vercel (Recommended)

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel

# Set environment variables in Vercel dashboard
```

### Netlify

```bash
# Install Netlify CLI
npm i -g netlify-cli

# Deploy
netlify deploy --prod

# Set environment variables in Netlify dashboard
```

## 🐛 Troubleshooting

### Wallet Not Connecting
- Ensure MetaMask is installed and unlocked
- Check that you're on Sapphire Testnet (chain ID: 0x5aff)
- Clear browser cache and try again

### Contract Calls Failing
- Verify `VITE_PRIVATE_PAYROLL_ADDRESS` is set correctly
- Ensure contract is deployed on Sapphire Testnet
- Check you have testnet tokens for gas

### Notary API Errors
- Verify `VITE_NOTARY_API_URL` is correct
- Check Notary service is running
- Application will use mock proofs if API is unavailable

## 📚 Architecture

This frontend follows the architecture specified in `MVP_ARCHITECTURE.md`:

- **Component-Based**: Modular, reusable components
- **Type-Safe**: Strict TypeScript throughout
- **State Management**: Zustand for global state
- **Web3 Integration**: Wagmi + Viem for Ethereum interactions
- **Premium UX**: Framer Motion animations, dark mode, responsive design

## 🤝 Contributing

This is a research prototype for the Oasis ROSE Bloom Grant. For questions or contributions, please refer to the main repository README.

## 📄 License

MIT License - See LICENSE file for details

---

**Built for Oasis Sapphire** 🌸 | **Powered by Confidential EVM** 🔐
