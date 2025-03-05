use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    clock::Clock,
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::Mint,
};

// Define program modules
pub mod error;
pub mod instruction;
pub mod metadata;
pub mod token_economy;
pub mod security;
pub mod events;
pub mod upgradable;
pub mod pump_fun;

// Use instruction definitions from our module
use instruction::AriaTokenInstruction;
use security::Role;
use error::AriaTokenError;

// Program entrypoint
entrypoint!(process_instruction);

// ARIA Token constants
const TOKEN_DECIMALS: u8 = 9;
const TOTAL_SUPPLY: u64 = 100_000_000_000_000_000; // 100 million tokens with 9 decimals

// 50% for user incentives
const USER_INCENTIVES: u64 = TOTAL_SUPPLY / 2;
// 20% for team development
const TEAM_DEVELOPMENT: u64 = TOTAL_SUPPLY / 5;
// 20% for community governance
const COMMUNITY_GOVERNANCE: u64 = TOTAL_SUPPLY / 5;
// 10% for marketing and partnerships
const MARKETING_PARTNERSHIPS: u64 = TOTAL_SUPPLY / 10;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AriaTokenInstruction {
    /// Initialize the ARIA token
    /// 0. `[writable]` The mint account to initialize
    /// 1. `[]` Rent sysvar
    /// 2. `[writable]` Token mint authority
    InitializeToken,

    /// Distribute tokens to different allocations
    /// 0. `[writable]` The mint account
    /// 1. `[writable]` User incentives token account
    /// 2. `[writable]` Team development token account
    /// 3. `[writable]` Community governance token account
    /// 4. `[writable]` Marketing partnerships token account
    /// 5. `[signer]` Mint authority
    DistributeTokens,
    
    /// Initialize token metadata
    /// 0. `[writable]` Metadata account
    /// 1. `[]` Mint account
    /// 2. `[signer]` Authority
    /// 3. `[]` Rent sysvar
    InitializeMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
    
    /// Update token metadata
    /// 0. `[writable]` Metadata account
    /// 1. `[signer]` Authority
    UpdateMetadata {
        name: Option<String>,
        symbol: Option<String>,
        uri: Option<String>,
    },
    
    /// Burn tokens
    /// 0. `[]` Mint account
    /// 1. `[writable]` Source account
    /// 2. `[signer]` Authority
    BurnTokens {
        amount: u64,
    },
    
    /// Lock tokens
    /// 0. `[writable]` Lock account
    /// 1. `[writable]` Source account
    /// 2. `[writable]` Vault account
    /// 3. `[signer]` Authority
    /// 4. `[]` Clock sysvar
    LockTokens {
        amount: u64,
        lock_duration: u64,
    },
    
    /// Unlock tokens
    /// 0. `[writable]` Lock account
    /// 1. `[writable]` Vault account
    /// 2. `[writable]` Destination account
    /// 3. `[signer]` Authority
    /// 4. `[]` Vault authority
    /// 5. `[]` Clock sysvar
    UnlockTokens,
    
    /// Initialize authority
    /// 0. `[writable]` Authority account
    /// 1. `[signer]` Primary admin
    InitializeAuthority,
    
    /// Add authority
    /// 0. `[writable]` Authority account
    /// 1. `[signer]` Admin
    AddAuthority {
        new_authority: Pubkey,
        role: Role,
    },
    
    /// Initialize program version
    /// 0. `[writable]` Version account
    /// 1. `[signer]` Authority
    InitializeVersion,
    
    /// Schedule program upgrade
    /// 0. `[writable]` Version account
    /// 1. `[signer]` Authority
    /// 2. `[]` Authority account for verification
    /// 3. `[]` Clock sysvar
    ScheduleUpgrade {
        new_program_id: Pubkey,
        upgrade_delay: u64,
    },
    
    /// Finalize program upgrade
    /// 0. `[writable]` Version account
    /// 1. `[signer]` Authority
    /// 2. `[]` Clock sysvar
    FinalizeUpgrade,
    
    /// Cancel program upgrade
    /// 0. `[writable]` Version account
    /// 1. `[signer]` Authority
    CancelUpgrade,
    
    /// Initialize pump.fun configuration
    /// 0. `[writable]` Config account
    /// 1. `[]` Mint account
    /// 2. `[signer]` Authority
    /// 3. `[]` Authority account for verification
    InitializePumpFunConfig {
        initial_price_usd: u64,
        admin_fee_bps: u16,
    },
    
    /// Update pump.fun configuration
    /// 0. `[writable]` Config account
    /// 1. `[signer]` Authority
    UpdatePumpFunConfig {
        initial_price_usd: Option<u64>,
        admin_fee_bps: Option<u16>,
        trading_enabled: Option<bool>,
        max_transaction_amount: Option<u64>,
        max_wallet_holdings: Option<u64>,
    },
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = AriaTokenInstruction::try_from_slice(instruction_data)?;

    match instruction {
        AriaTokenInstruction::InitializeToken => {
            msg!("Instruction: Initialize Token");
            initialize_token(program_id, accounts)
        },
        AriaTokenInstruction::DistributeTokens => {
            msg!("Instruction: Distribute Tokens");
            distribute_tokens(program_id, accounts)
        },
        AriaTokenInstruction::InitializeMetadata { name, symbol, uri } => {
            msg!("Instruction: Initialize Metadata");
            metadata::initialize_metadata(program_id, accounts, name, symbol, uri)
        },
        AriaTokenInstruction::UpdateMetadata { name, symbol, uri } => {
            msg!("Instruction: Update Metadata");
            metadata::update_metadata(program_id, accounts, name, symbol, uri)
        },
        AriaTokenInstruction::BurnTokens { amount } => {
            msg!("Instruction: Burn Tokens");
            token_economy::burn_tokens(program_id, accounts, amount)
        },
        AriaTokenInstruction::LockTokens { amount, lock_duration } => {
            msg!("Instruction: Lock Tokens");
            token_economy::lock_tokens(program_id, accounts, amount, lock_duration)
        },
        AriaTokenInstruction::UnlockTokens => {
            msg!("Instruction: Unlock Tokens");
            token_economy::unlock_tokens(program_id, accounts)
        },
        AriaTokenInstruction::InitializeAuthority => {
            msg!("Instruction: Initialize Authority");
            security::initialize_authority(program_id, accounts)
        },
        AriaTokenInstruction::AddAuthority { new_authority, role } => {
            msg!("Instruction: Add Authority");
            security::add_authority(program_id, accounts, new_authority, role)
        },
        AriaTokenInstruction::InitializeVersion => {
            msg!("Instruction: Initialize Version");
            upgradable::initialize_version(program_id, accounts)
        },
        AriaTokenInstruction::ScheduleUpgrade { new_program_id, upgrade_delay } => {
            msg!("Instruction: Schedule Upgrade");
            upgradable::schedule_upgrade(program_id, accounts, new_program_id, upgrade_delay)
        },
        AriaTokenInstruction::FinalizeUpgrade => {
            msg!("Instruction: Finalize Upgrade");
            upgradable::finalize_upgrade(program_id, accounts)
        },
        AriaTokenInstruction::CancelUpgrade => {
            msg!("Instruction: Cancel Upgrade");
            upgradable::cancel_upgrade(program_id, accounts)
        },
        AriaTokenInstruction::InitializePumpFunConfig { initial_price_usd, admin_fee_bps } => {
            msg!("Instruction: Initialize pump.fun Config");
            pump_fun::initialize_pump_fun_config(program_id, accounts, initial_price_usd, admin_fee_bps)
        },
        AriaTokenInstruction::UpdatePumpFunConfig { 
            initial_price_usd, 
            admin_fee_bps, 
            trading_enabled,
            max_transaction_amount,
            max_wallet_holdings,
        } => {
            msg!("Instruction: Update pump.fun Config");
            pump_fun::update_pump_fun_config(
                program_id, 
                accounts, 
                initial_price_usd, 
                admin_fee_bps, 
                trading_enabled,
                max_transaction_amount,
                max_wallet_holdings,
            )
        },
    }
}

fn initialize_token(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_info_iter)?;
    let rent_account = next_account_info(account_info_iter)?;
    let mint_authority = next_account_info(account_info_iter)?;

    // Ensure the mint account is owned by the token program
    if mint_account.owner != &spl_token::id() {
        msg!("Mint account is not owned by the token program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = Rent::from_account_info(rent_account)?;
    
    // Check if the mint account has enough lamports to be rent-exempt
    if !rent.is_exempt(mint_account.lamports(), Mint::LEN) {
        msg!("Mint account is not rent-exempt");
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Initialize the mint account
    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(),
        mint_account.key,
        mint_authority.key,
        Some(mint_authority.key), // Freeze authority (same as mint authority for now)
        TOKEN_DECIMALS,
    )?;

    // Log the event
    events::log_token_initialized(mint_account.key, mint_authority.key, TOKEN_DECIMALS);

    msg!("ARIA Token initialized successfully");
    
    Ok(())
}

fn distribute_tokens(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_info_iter)?;
    let user_incentives_account = next_account_info(account_info_iter)?;
    let team_development_account = next_account_info(account_info_iter)?;
    let community_governance_account = next_account_info(account_info_iter)?;
    let marketing_partnerships_account = next_account_info(account_info_iter)?;
    let mint_authority = next_account_info(account_info_iter)?;

    // Ensure the mint authority is a signer
    if !mint_authority.is_signer {
        msg!("Mint authority must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Mint tokens to user incentives allocation
    let user_incentives_instruction = mint_to(
        &spl_token::id(),
        mint_account.key,
        user_incentives_account.key,
        mint_authority.key,
        &[],
        USER_INCENTIVES,
    )?;
    
    // Mint tokens to team development allocation
    let team_development_instruction = mint_to(
        &spl_token::id(),
        mint_account.key,
        team_development_account.key,
        mint_authority.key,
        &[],
        TEAM_DEVELOPMENT,
    )?;
    
    // Mint tokens to community governance allocation
    let community_governance_instruction = mint_to(
        &spl_token::id(),
        mint_account.key,
        community_governance_account.key,
        mint_authority.key,
        &[],
        COMMUNITY_GOVERNANCE,
    )?;
    
    // Mint tokens to marketing and partnerships allocation
    let marketing_partnerships_instruction = mint_to(
        &spl_token::id(),
        mint_account.key,
        marketing_partnerships_account.key,
        mint_authority.key,
        &[],
        MARKETING_PARTNERSHIPS,
    )?;

    // Log the distribution event
    events::log_token_distributed(
        mint_account.key,
        USER_INCENTIVES,
        TEAM_DEVELOPMENT,
        COMMUNITY_GOVERNANCE,
        MARKETING_PARTNERSHIPS,
    );

    msg!("ARIA Tokens distributed successfully");
    
    Ok(())
} 