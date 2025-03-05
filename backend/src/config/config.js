/**
 * Configuration for the ARIA backend
 */
const config = {
  // Server configuration
  server: {
    port: process.env.PORT || 3000,
    environment: process.env.NODE_ENV || 'development'
  },
  
  // Database configuration
  database: {
    url: process.env.MONGODB_URI || 'mongodb://localhost:27017/aria',
    options: {
      useNewUrlParser: true,
      useUnifiedTopology: true
    }
  },
  
  // Solana blockchain configuration
  solana: {
    rpcUrl: process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com',
    tokenMint: process.env.ARIA_TOKEN_MINT || '11111111111111111111111111111111',
    keyPath: process.env.ADMIN_KEY_PATH || './keys/admin.json'
  },
  
  // Authentication configuration
  auth: {
    jwtSecret: process.env.JWT_SECRET || 'aria-dev-secret-key',
    jwtExpiration: '24h'
  },
  
  // Data collection configuration
  dataCollection: {
    syncInterval: 60 * 60 * 1000, // 1 hour
    minRewardAmount: 0.1,
    maxRewardAmount: 5.0
  }
};

module.exports = config; 