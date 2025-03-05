use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
    clock,
};
use crate::security::Role;

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
}

pub fn initialize_token(
    program_id: &Pubkey,
    mint_account: &Pubkey,
    mint_authority: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*mint_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(*mint_authority, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::InitializeToken,
        accounts,
    )
}

pub fn distribute_tokens(
    program_id: &Pubkey,
    mint_account: &Pubkey,
    user_incentives_account: &Pubkey,
    team_development_account: &Pubkey,
    community_governance_account: &Pubkey,
    marketing_partnerships_account: &Pubkey,
    mint_authority: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*mint_account, false),
        AccountMeta::new(*user_incentives_account, false),
        AccountMeta::new(*team_development_account, false),
        AccountMeta::new(*community_governance_account, false),
        AccountMeta::new(*marketing_partnerships_account, false),
        AccountMeta::new_readonly(*mint_authority, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::DistributeTokens,
        accounts,
    )
}

pub fn initialize_metadata(
    program_id: &Pubkey,
    metadata_account: &Pubkey,
    mint_account: &Pubkey,
    authority: &Pubkey,
    name: String,
    symbol: String,
    uri: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*metadata_account, false),
        AccountMeta::new_readonly(*mint_account, false),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::InitializeMetadata { name, symbol, uri },
        accounts,
    )
}

pub fn update_metadata(
    program_id: &Pubkey,
    metadata_account: &Pubkey,
    authority: &Pubkey,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*metadata_account, false),
        AccountMeta::new_readonly(*authority, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::UpdateMetadata { name, symbol, uri },
        accounts,
    )
}

pub fn burn_tokens(
    program_id: &Pubkey,
    mint_account: &Pubkey,
    source_account: &Pubkey,
    authority: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*mint_account, false),
        AccountMeta::new(*source_account, false),
        AccountMeta::new_readonly(*authority, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::BurnTokens { amount },
        accounts,
    )
}

pub fn lock_tokens(
    program_id: &Pubkey,
    lock_account: &Pubkey,
    source_account: &Pubkey,
    vault_account: &Pubkey,
    authority: &Pubkey,
    amount: u64,
    lock_duration: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*lock_account, false),
        AccountMeta::new(*source_account, false),
        AccountMeta::new(*vault_account, false),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new_readonly(clock::id(), false),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::LockTokens { amount, lock_duration },
        accounts,
    )
}

pub fn unlock_tokens(
    program_id: &Pubkey,
    lock_account: &Pubkey,
    vault_account: &Pubkey,
    destination_account: &Pubkey,
    authority: &Pubkey,
    vault_authority: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*lock_account, false),
        AccountMeta::new(*vault_account, false),
        AccountMeta::new(*destination_account, false),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new_readonly(*vault_authority, false),
        AccountMeta::new_readonly(clock::id(), false),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::UnlockTokens,
        accounts,
    )
}

pub fn initialize_authority(
    program_id: &Pubkey,
    authority_account: &Pubkey,
    primary_admin: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*authority_account, false),
        AccountMeta::new_readonly(*primary_admin, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::InitializeAuthority,
        accounts,
    )
}

pub fn add_authority(
    program_id: &Pubkey,
    authority_account: &Pubkey,
    admin: &Pubkey,
    new_authority: Pubkey,
    role: Role,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*authority_account, false),
        AccountMeta::new_readonly(*admin, true),
    ];

    Instruction::new_with_borsh(
        *program_id,
        &AriaTokenInstruction::AddAuthority { new_authority, role },
        accounts,
    )
} 