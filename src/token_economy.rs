use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Clock,
    sysvar::Sysvar,
};
use spl_token::instruction::{burn, transfer};

/// Token locking data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenLock {
    /// Owner of the locked tokens
    pub owner: Pubkey,
    /// Amount of tokens locked
    pub amount: u64,
    /// Unix timestamp when tokens can be unlocked
    pub unlock_time: u64,
    /// Whether tokens have been claimed after unlock
    pub is_claimed: bool,
}

/// Burn tokens to reduce supply
pub fn burn_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_info_iter)?;
    let source_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Create and send the burn instruction
    let burn_instruction = burn(
        &spl_token::id(),
        source_account.key,
        mint_account.key,
        authority.key,
        &[],
        amount,
    )?;

    msg!("Burned {} ARIA tokens", amount);
    
    Ok(())
}

/// Lock tokens for a specific time period
pub fn lock_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    lock_duration: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lock_account = next_account_info(account_info_iter)?;
    let source_account = next_account_info(account_info_iter)?;
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if lock account is owned by our program
    if lock_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the current clock
    let clock = Clock::from_account_info(clock_sysvar)?;
    
    // Create the lock data
    let lock_data = TokenLock {
        owner: *authority.key,
        amount,
        unlock_time: clock.unix_timestamp as u64 + lock_duration,
        is_claimed: false,
    };

    // Transfer tokens to vault
    let transfer_instruction = transfer(
        &spl_token::id(),
        source_account.key,
        vault_account.key,
        authority.key,
        &[],
        amount,
    )?;

    // Save lock data
    lock_data.serialize(&mut *lock_account.data.borrow_mut())?;

    msg!("Locked {} ARIA tokens until timestamp {}", amount, lock_data.unlock_time);
    
    Ok(())
}

/// Unlock tokens after lock period
pub fn unlock_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let lock_account = next_account_info(account_info_iter)?;
    let vault_account = next_account_info(account_info_iter)?;
    let destination_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let vault_authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if lock account is owned by our program
    if lock_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get lock data
    let mut lock_data = TokenLock::try_from_slice(&lock_account.data.borrow())?;

    // Check if caller is the owner of locked tokens
    if &lock_data.owner != authority.key {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if tokens are already claimed
    if lock_data.is_claimed {
        return Err(ProgramError::InvalidAccountData);
    }

    // Get the current clock
    let clock = Clock::from_account_info(clock_sysvar)?;
    
    // Check if lock period has passed
    if (clock.unix_timestamp as u64) < lock_data.unlock_time {
        msg!("Tokens are still locked until timestamp {}", lock_data.unlock_time);
        return Err(ProgramError::InvalidAccountData);
    }

    // Transfer tokens from vault to destination
    let transfer_instruction = transfer(
        &spl_token::id(),
        vault_account.key,
        destination_account.key,
        vault_authority.key,
        &[],
        lock_data.amount,
    )?;

    // Mark tokens as claimed
    lock_data.is_claimed = true;
    lock_data.serialize(&mut *lock_account.data.borrow_mut())?;

    msg!("Unlocked {} ARIA tokens", lock_data.amount);
    
    Ok(())
} 