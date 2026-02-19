# Oasis Sapphire Contract Deployment Guide

## Prerequisites

1. **Get Testnet Tokens**:
   - Visit: https://faucet.testnet.oasis.io/
   - Enter your wallet address
   - Receive TEST tokens for gas fees

2. **Set Up Environment Variables**:
   ```bash
   cd contracts/oasis
   # Edit .env file and fill in:
   # - PRIVATE_KEY: Your deployer wallet private key (without 0x)
   # - TRUSTED_NOTARY_ADDRESS: For testing, use your wallet address
   ```

## Deployment Steps

### 1. Compile Contract
```bash
npx hardhat compile
```

### 2. Deploy to Sapphire Testnet
```bash
npx hardhat run scripts/deploy.js --network sapphire_testnet
```

### 3. Copy Contract Address
After successful deployment, you'll see:
```
✅ PrivatePayroll Contract Deployed Successfully!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 Contract Address: 0x...
🔗 Explorer: https://testnet.explorer.sapphire.oasis.io/address/0x...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 4. Update Frontend
```bash
cd ../../frontend
# Edit .env file:
VITE_PRIVATE_PAYROLL_ADDRESS=0xYourDeployedContractAddress
```

### 5. Restart Frontend
```bash
npm run dev
```

## Troubleshooting

### "Insufficient funds" error
- Get more testnet tokens from the faucet
- Check your wallet balance

### "Invalid notary address" error
- Make sure TRUSTED_NOTARY_ADDRESS is set in .env
- Use your wallet address for testing

### Network connection issues
- Check SAPPHIRE_RPC_URL in .env
- Default: https://testnet.sapphire.oasis.io

## Verification

After deployment, verify the contract on the explorer:
1. Visit the Explorer link from deployment output
2. Check the "Contract" tab
3. Verify TRUSTED_NOTARY address is correct

## Next Steps

1. ✅ Deploy PrivatePayroll contract
2. Update frontend .env with contract address
3. Test wallet connection
4. Test proof generation (mock mode)
5. Test blockchain submission
6. View encrypted salary

---

**Network**: Oasis Sapphire Testnet  
**Chain ID**: 23295 (0x5aff)  
**Explorer**: https://testnet.explorer.sapphire.oasis.io
