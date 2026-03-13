#!/bin/bash
set -e

# Turn on bold green colors
GREEN='\033[1;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}[SYSTEM] INITIATING UPE MVP DEMO ENVIRONMENT...${NC}"

# Load environment variables if .env exists
if [ -f .env ]; then
  echo "Loading root .env variables..."
  export $(grep -v '^#' .env | xargs)
fi

echo "Booting Next.js frontend..."

cd apps/web
if [ -f .env.local ]; then
  echo "Loading apps/web/.env.local variables..."
  export $(grep -v '^#' .env.local | xargs)
fi

# Run the development server
npm run dev
