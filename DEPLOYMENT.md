# ARIA Token Deployment Guide

This guide explains how to deploy the ARIA token contract to Solana and prepare it for listing on pump.fun.

## Prerequisites

Before you begin, ensure you have the following installed:

1. [Rust](https://www.rust-lang.org/tools/install) and Cargo
2. [Solana CLI Tools](https://docs.solana.com/cli/install-solana-cli-tools)
3. [Node.js](https://nodejs.org/) (v14 or later) and npm

## Setup Environment

1. Configure your Solana CLI:

```bash
# Set to devnet for testing
solana config set --url https://api.devnet.solana.com

# Generate a new keypair (if you don't have one)
solana-keygen new --outfile ~/.config/solana/devnet.json

# Set this keypair as default
solana config set --keypair ~/.config/solana/devnet.json
```

2. Fund your account:

```bash
solana airdrop 2 # Request 2 SOL from devnet faucet
```

## Build the Token Program

1. Compile the Rust program:

```bash
# From the project root
cargo build-bpf
```

This creates the compiled program at `target/deploy/aria_token.so`.

## Deploy the Token Program

1. Deploy the program to devnet:

```bash
solana program deploy target/deploy/aria_token.so
```

2. Save the program ID that is displayed after successful deployment.

## Initialize the Token

1. Install Node.js dependencies:

```bash
npm install
```

2. Update the `scripts/deploy.js` file with your program ID.

3. Run the deployment script:

```bash
npm run deploy
```

This script will:
- Create a token mint
- Initialize the ARIA token with 9 decimals
- Create token accounts for each allocation
- Mint tokens according to the distribution plan

## Preparing for pump.fun Listing

1. Export the token mint ID from the deployment:

```bash
# The token mint address is displayed at the end of the deploy script
```

2. Create metadata for your token by following pump.fun's requirements, which typically include:
   - Token name (ARIA Token)
   - Token symbol (ARI)
   - Token description
   - Logo image
   - Website URL
   - Social media links

3. Visit pump.fun and connect your wallet that has control of the token mint.

4. Create a new token listing using the token mint address and metadata prepared above.

5. Configure the initial price, trading parameters, and liquidity settings.

## Mainnet Deployment

When you're ready to deploy to mainnet:

1. Update your Solana configuration:

```bash
solana config set --url https://api.mainnet-beta.solana.com
solana config set --keypair [path-to-your-mainnet-keypair]
```

2. Ensure your mainnet account is funded with sufficient SOL.

3. Follow the same deployment steps as for devnet, but use your mainnet account.

4. Create the official pump.fun listing with your mainnet token.

## Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Documentation](https://spl.solana.com/token)
- [pump.fun Documentation](https://docs.pump.fun/) 