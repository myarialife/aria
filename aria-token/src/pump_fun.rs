use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::{AriaTokenError, check_account_owner, check_signer};
use crate::security::authorize;

/// Configuration for pump.fun listing
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpFunConfig {
    /// Token mint
    pub mint: Pubkey,
    /// Initial price in USD (as fixed point with 6 decimals)
    pub initial_price_usd: u64,
    /// Admin fee basis points (100 = 1%)
    pub admin_fee_bps: u16,
    /// Trading enabled flag
    pub trading_enabled: bool,
    /// Maximum transaction amount (if any)
    pub max_transaction_amount: Option<u64>,
    /// Maximum wallet holdings (if any)
    pub max_wallet_holdings: Option<u64>,
    /// Authority who can update the configuration
    pub authority: Pubkey,
}

impl PumpFunConfig {
    pub fn new(
        mint: Pubkey, 
        initial_price_usd: u64, 
        admin_fee_bps: u16,
        authority: Pubkey
    ) -> Self {
        Self {
            mint,
            initial_price_usd,
            admin_fee_bps,
            trading_enabled: false,  // Default to trading disabled
            max_transaction_amount: None,
            max_wallet_holdings: None,
            authority,
        }
    }
}

/// Initialize pump.fun configuration
pub fn initialize_pump_fun_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_price_usd: u64,
    admin_fee_bps: u16,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let config_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(config_account, program_id)?;
    check_signer(authority)?;
    
    // Check if the authority has admin role
    authorize(authority, authority_account, 0)?; // Admin role = 0
    
    if !config_account.data_is_empty() {
        return Err(AriaTokenError::InvalidMetadata.into());
    }
    
    // Create new pump.fun configuration
    let config = PumpFunConfig::new(
        *mint_account.key,
        initial_price_usd,
        admin_fee_bps,
        *authority.key
    );
    
    // Serialize and store config in account
    config.serialize(&mut *config_account.data.borrow_mut())?;
    
    msg!("pump.fun configuration initialized for mint {}", mint_account.key);
    msg!("Initial price: {} USD (fixed point), Admin fee: {}bps", 
        initial_price_usd, 
        admin_fee_bps
    );
    
    Ok(())
}

/// Update pump.fun configuration
pub fn update_pump_fun_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_price_usd: Option<u64>,
    admin_fee_bps: Option<u16>,
    trading_enabled: Option<bool>,
    max_transaction_amount: Option<u64>,
    max_wallet_holdings: Option<u64>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let config_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(config_account, program_id)?;
    check_signer(authority)?;
    
    // Load config account data
    let mut config = PumpFunConfig::deserialize(&mut &config_account.data.borrow()[..])?;
    
    // Check if the signer is the authorized authority
    if *authority.key != config.authority {
        return Err(AriaTokenError::IncorrectAuthority.into());
    }
    
    // Update fields if provided
    if let Some(price) = initial_price_usd {
        config.initial_price_usd = price;
        msg!("Updated initial price to {} USD", price);
    }
    
    if let Some(fee) = admin_fee_bps {
        config.admin_fee_bps = fee;
        msg!("Updated admin fee to {}bps", fee);
    }
    
    if let Some(enabled) = trading_enabled {
        config.trading_enabled = enabled;
        msg!("Trading {}", if enabled { "enabled" } else { "disabled" });
    }
    
    if let Some(max_tx) = max_transaction_amount {
        config.max_transaction_amount = Some(max_tx);
        msg!("Maximum transaction amount set to {}", max_tx);
    }
    
    if let Some(max_holdings) = max_wallet_holdings {
        config.max_wallet_holdings = Some(max_holdings);
        msg!("Maximum wallet holdings set to {}", max_holdings);
    }
    
    // Save updated configuration
    config.serialize(&mut *config_account.data.borrow_mut())?;
    
    msg!("pump.fun configuration updated successfully");
    
    Ok(())
}
