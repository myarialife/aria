const { Connection, PublicKey, Transaction } = require('@solana/web3.js');
const { Token, TOKEN_PROGRAM_ID } = require('@solana/spl-token');
const fs = require('fs');
const path = require('path');
const logger = require('../utils/logger');

class WalletService {
  constructor() {
    // Initialize Solana connection
    this.connection = new Connection(process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com', 'confirmed');
    
    logger.info('Initialized Solana connection to: ' + (process.env.SOLANA_RPC_URL || 'https://api.devnet.solana.com'));
    
    // ARIA token mint address
    this.ariaMint = new PublicKey(process.env.ARIA_TOKEN_MINT || '11111111111111111111111111111111');
    
    // Load admin wallet (for development only, production should use more secure handling)
    try {
      if (process.env.NODE_ENV === 'development') {
        const keyPath = path.join(__dirname, '../../keys/admin.json');
        const keyData = JSON.parse(fs.readFileSync(keyPath, 'utf-8'));
        this.adminWallet = keyData;
      } else {
        // Production environment, get from environment variables
        this.adminWallet = {
          publicKey: process.env.ADMIN_WALLET_PUBLIC_KEY,
          secretKey: process.env.ADMIN_WALLET_SECRET_KEY
        };
      }
      logger.info('Admin wallet loaded successfully');
    } catch (error) {
      logger.error('Failed to load admin wallet', error);
      throw new Error('Failed to initialize wallet service');
    }
  }

  /**
   * Get token balance for a wallet address
   * @param {string} walletAddress - Solana wallet address
   * @returns {Promise<number>} Token balance
   */
  async getTokenBalance(walletAddress) {
    try {
      const walletPublicKey = new PublicKey(walletAddress);
      const token = new Token(this.connection, this.ariaMint, TOKEN_PROGRAM_ID, this.adminWallet);
      
      // Find token account
      const tokenAccounts = await this.connection.getTokenAccountsByOwner(walletPublicKey, { mint: this.ariaMint });
      
      // Get balance
      let balance = 0;
      if (tokenAccounts.value.length > 0) {
        const tokenAccount = tokenAccounts.value[0];
        const accountInfo = await token.getAccountInfo(tokenAccount.pubkey);
        balance = accountInfo.amount.toNumber() / Math.pow(10, token.decimals);
      }
      
      return balance;
    } catch (error) {
      logger.error(`Error getting token balance for ${walletAddress}`, error);
      throw error;
    }
  }

  /**
   * Send token reward to user
   * @param {string} destinationAddress - Recipient wallet address
   * @param {number} amount - Amount of tokens to send
   * @returns {Promise<{success: boolean, txId: string}>} Transaction result
   */
  async sendReward(destinationAddress, amount) {
    try {
      // Initialize token
      const token = new Token(
        this.connection,
        this.ariaMint,
        TOKEN_PROGRAM_ID,
        this.adminWallet
      );
      
      const destPublicKey = new PublicKey(destinationAddress);
      
      // Find or create destination account
      const associatedAccount = await token.getOrCreateAssociatedAccountInfo(
        destPublicKey
      );
      
      // Transfer transaction
      const transaction = new Transaction().add(
        Token.createTransferInstruction(
          TOKEN_PROGRAM_ID,
          this.adminWallet.publicKey,
          associatedAccount.address,
          this.adminWallet.publicKey,
          [],
          amount * Math.pow(10, token.decimals)
        )
      );
      
      // Send transaction
      const signature = await this.connection.sendTransaction(
        transaction,
        [this.adminWallet],
        { skipPreflight: false }
      );
      
      await this.connection.confirmTransaction(signature);
      
      logger.info(`Successfully sent ${amount} tokens to ${destinationAddress}, txId: ${signature}`);
      
      return {
        success: true,
        txId: signature
      };
    } catch (error) {
      logger.error(`Error sending reward to ${destinationAddress}`, error);
      return {
        success: false,
        error: error.message
      };
    }
  }

  /**
   * Batch send rewards to multiple addresses
   * @param {Array<{address: string, amount: number}>} rewards - Array of reward objects
   * @returns {Promise<Array<{address: string, success: boolean, txId: string, amount: number}>>} Results
   */
  async batchSendRewards(rewards) {
    const results = [];
    
    for (const reward of rewards) {
      const result = await this.sendReward(reward.address, reward.amount);
      results.push({
        address: reward.address,
        success: result.success,
        txId: result.txId,
        amount: reward.amount,
        error: result.error
      });
    }
    
    return results;
  }
}

module.exports = new WalletService();