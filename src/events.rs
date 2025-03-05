use solana_program::{
    msg,
    pubkey::Pubkey,
};
use borsh::{BorshSerialize, BorshDeserialize};

/// Event types for ARIA token
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum AriaEvent {
    /// Token initialized
    TokenInitialized {
        mint: Pubkey,
        authority: Pubkey,
        decimals: u8,
    },
    
    /// Token distribution
    TokenDistributed {
        mint: Pubkey,
        user_incentives_amount: u64,
        team_development_amount: u64,
        community_governance_amount: u64,
        marketing_partnerships_amount: u64,
    },
    
    /// Metadata updated
    MetadataUpdated {
        mint: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    },
    
    /// Tokens burned
    TokensBurned {
        mint: Pubkey,
        source: Pubkey,
        amount: u64,
    },
    
    /// Tokens locked
    TokensLocked {
        lock_account: Pubkey,
        owner: Pubkey,
        amount: u64,
        unlock_time: u64,
    },
    
    /// Tokens unlocked
    TokensUnlocked {
        lock_account: Pubkey,
        owner: Pubkey,
        destination: Pubkey,
        amount: u64,
    },
    
    /// Authority added
    AuthorityAdded {
        authority: Pubkey,
        role: u8,
        admin: Pubkey,
    },
    
    /// Authority removed
    AuthorityRemoved {
        authority: Pubkey,
        role: u8,
        admin: Pubkey,
    },
}

/// Log an event to the Solana logs
pub fn log_event(event: &AriaEvent) {
    let event_data = borsh::to_vec(event).unwrap_or_default();
    let event_data_b58 = bs58::encode(event_data).into_string();
    
    // Log with a special prefix for easier parsing
    msg!("ARIA-EVENT:{}", event_data_b58);
}

/// Log token initialization event
pub fn log_token_initialized(mint: &Pubkey, authority: &Pubkey, decimals: u8) {
    let event = AriaEvent::TokenInitialized {
        mint: *mint,
        authority: *authority,
        decimals,
    };
    log_event(&event);
}

/// Log token distribution event
pub fn log_token_distributed(
    mint: &Pubkey,
    user_incentives_amount: u64,
    team_development_amount: u64,
    community_governance_amount: u64,
    marketing_partnerships_amount: u64,
) {
    let event = AriaEvent::TokenDistributed {
        mint: *mint,
        user_incentives_amount,
        team_development_amount,
        community_governance_amount,
        marketing_partnerships_amount,
    };
    log_event(&event);
}

/// Log metadata updated event
pub fn log_metadata_updated(mint: &Pubkey, name: &str, symbol: &str, uri: &str) {
    let event = AriaEvent::MetadataUpdated {
        mint: *mint,
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    };
    log_event(&event);
}

/// Log tokens burned event
pub fn log_tokens_burned(mint: &Pubkey, source: &Pubkey, amount: u64) {
    let event = AriaEvent::TokensBurned {
        mint: *mint,
        source: *source,
        amount,
    };
    log_event(&event);
}

/// Log tokens locked event
pub fn log_tokens_locked(lock_account: &Pubkey, owner: &Pubkey, amount: u64, unlock_time: u64) {
    let event = AriaEvent::TokensLocked {
        lock_account: *lock_account,
        owner: *owner,
        amount,
        unlock_time,
    };
    log_event(&event);
}

/// Log tokens unlocked event
pub fn log_tokens_unlocked(lock_account: &Pubkey, owner: &Pubkey, destination: &Pubkey, amount: u64) {
    let event = AriaEvent::TokensUnlocked {
        lock_account: *lock_account,
        owner: *owner,
        destination: *destination,
        amount,
    };
    log_event(&event);
}

/// Log authority added event
pub fn log_authority_added(authority: &Pubkey, role: u8, admin: &Pubkey) {
    let event = AriaEvent::AuthorityAdded {
        authority: *authority,
        role,
        admin: *admin,
    };
    log_event(&event);
}

/// Log authority removed event
pub fn log_authority_removed(authority: &Pubkey, role: u8, admin: &Pubkey) {
    let event = AriaEvent::AuthorityRemoved {
        authority: *authority,
        role,
        admin: *admin,
    };
    log_event(&event);
} 