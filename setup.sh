#!/bin/bash
# Universal Privacy Engine — Local Development Setup
set -e

echo "🔐 Universal Privacy Engine — Setup"
echo "===================================="
echo ""

# 1. Rust Notary Service
echo "📦 Building Rust Notary Service..."
cd core
cp .env.example .env 2>/dev/null || true
echo "  → Edit core/.env and set NOTARY_PRIVATE_KEY"
echo ""

# 2. Start the notary
echo "🚀 To start the Notary API:"
echo "   cd core && PORT=3002 cargo run --release"
echo ""

# 3. Expose via Cloudflare Tunnel (no account needed)
echo "🌐 To expose via Cloudflare Tunnel:"
echo "   cloudflared tunnel --url http://localhost:3002"
echo "   → Copy the https://XXXX.trycloudflare.com URL"
echo ""

# 4. Frontend
echo "💻 To start the Frontend:"
echo "   cd frontend"
echo "   cp .env.example .env"
echo "   # Set VITE_NOTARY_API_URL to your tunnel URL"
echo "   npm install && npm run dev"
echo ""

echo "✅ See README.md for full instructions."
