use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum AriaTokenError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    
    #[error("Not rent exempt")]
    NotRentExempt,
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Incorrect authority")]
    IncorrectAuthority,

    #[error("Operation not permitted")]
    OperationNotPermitted,
    
    #[error("Token account is frozen")]
    AccountFrozen,
    
    #[error("Token account owner mismatch")]
    OwnerMismatch,
    
    #[error("Invalid token mint")]
    InvalidMint,
    
    #[error("Tokens are still locked")]
    StillLocked,
    
    #[error("Tokens already claimed")]
    AlreadyClaimed,
    
    #[error("Invalid metadata")]
    InvalidMetadata,
    
    #[error("Authority not found")]
    AuthorityNotFound,
    
    #[error("Role not assigned")]
    RoleNotAssigned,
    
    #[error("Version mismatch")]
    VersionMismatch,
    
    #[error("Upgrade in progress")]
    UpgradeInProgress,
    
    #[error("No upgrade scheduled")]
    NoUpgradeScheduled,
    
    #[error("Upgrade time not reached")]
    UpgradeTimeNotReached,
    
    #[error("Trading not enabled")]
    TradingNotEnabled,
    
    #[error("Transaction exceeds maximum limit")]
    ExceedsMaxTransaction,
    
    #[error("Wallet holdings exceed maximum limit")]
    ExceedsMaxWalletHoldings,
    
    #[error("Operation timeout")]
    OperationTimeout,
    
    #[error("Emergency pause active")]
    EmergencyPauseActive,
}

/// Maps AriaTokenError to ProgramError
impl From<AriaTokenError> for ProgramError {
    fn from(e: AriaTokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

/// Utility function to check if an account owner is the expected program
pub fn check_account_owner(
    account_info: &solana_program::account_info::AccountInfo,
    program_id: &solana_program::pubkey::Pubkey,
) -> Result<(), AriaTokenError> {
    if account_info.owner != program_id {
        solana_program::msg!("Account owner is invalid");
        return Err(AriaTokenError::OwnerMismatch);
    }
    Ok(())
}

/// Utility function to check if an account is a signer
pub fn check_signer(
    account_info: &solana_program::account_info::AccountInfo,
) -> Result<(), AriaTokenError> {
    if !account_info.is_signer {
        solana_program::msg!("Required signature is missing");
        return Err(AriaTokenError::IncorrectAuthority);
    }
    Ok(())
}

/// Utility function to check if an account is rent-exempt
pub fn check_rent_exempt(
    rent: &solana_program::rent::Rent,
    account_info: &solana_program::account_info::AccountInfo,
) -> Result<(), AriaTokenError> {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        solana_program::msg!("Account is not rent-exempt");
        return Err(AriaTokenError::NotRentExempt);
    }
    Ok(())
} 