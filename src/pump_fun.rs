use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::security::{Role, check_authority};

/// pump.fun listing configuration
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
            trading_enabled: false, // Start with trading disabled
            max_transaction_amount: None,
            max_wallet_holdings: None,
            authority,
        }
    }
}

/// Initialize pump.fun listing configuration
pub fn initialize_pump_fun_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_price_usd: u64,
    admin_fee_bps: u16,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check if authority has admin role
    if !check_authority(&[authority_account.clone(), authority.clone()], Role::Admin)? {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if config account is owned by our program
    if config_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Create new pump.fun configuration
    let config = PumpFunConfig::new(
        *mint_account.key,
        initial_price_usd,
        admin_fee_bps,
        *authority.key,
    );
    
    // Serialize and save the configuration
    config.serialize(&mut *config_account.data.borrow_mut())?;
    
    msg!(
        "pump.fun listing configuration initialized: price ${}.{:06}, fee {}%",
        initial_price_usd / 1_000_000,
        initial_price_usd % 1_000_000,
        admin_fee_bps as f32 / 100.0
    );
    
    Ok(())
}

/// Update pump.fun listing configuration
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
    let config_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check if config account is owned by our program
    if config_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize configuration
    let mut config = PumpFunConfig::try_from_slice(&config_account.data.borrow())?;
    
    // Check if signer is the authority
    if config.authority != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Update fields if provided
    if let Some(price) = initial_price_usd {
        config.initial_price_usd = price;
    }
    
    if let Some(fee) = admin_fee_bps {
        config.admin_fee_bps = fee;
    }
    
    if let Some(enabled) = trading_enabled {
        config.trading_enabled = enabled;
    }
    
    if max_transaction_amount.is_some() {
        config.max_transaction_amount = max_transaction_amount;
    }
    
    if max_wallet_holdings.is_some() {
        config.max_wallet_holdings = max_wallet_holdings;
    }
    
    // Serialize and save the updated configuration
    config.serialize(&mut *config_account.data.borrow_mut())?;
    
    msg!("pump.fun listing configuration updated");
    
    if let Some(enabled) = trading_enabled {
        if enabled {
            msg!("Trading enabled on pump.fun");
        } else {
            msg!("Trading disabled on pump.fun");
        }
    }
    
    Ok(())
} 