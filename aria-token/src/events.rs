use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    pubkey::Pubkey,
    msg,
};

/// Enum representing all possible events that can be emitted from the ARIA token contract
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

/// Log an event by serializing it and emitting the data through the msg! macro
pub fn log_event(event: &AriaEvent) {
    let data = event.try_to_vec().unwrap_or_default();
    let base58_data = bs58::encode(&data).into_string();
    msg!("ARIA-EVENT:{}", base58_data);
}

/// Log token initialization event
pub fn log_token_initialized(mint: &Pubkey, authority: &Pubkey, decimals: u8) {
    log_event(&AriaEvent::TokenInitialized {
        mint: *mint,
        authority: *authority,
        decimals,
    });
}

/// Log token distribution event
pub fn log_token_distributed(
    mint: &Pubkey,
    user_incentives_amount: u64,
    team_development_amount: u64,
    community_governance_amount: u64,
    marketing_partnerships_amount: u64,
) {
    log_event(&AriaEvent::TokenDistributed {
        mint: *mint,
        user_incentives_amount,
        team_development_amount,
        community_governance_amount,
        marketing_partnerships_amount,
    });
}

/// Log metadata update event
pub fn log_metadata_updated(mint: &Pubkey, name: &str, symbol: &str, uri: &str) {
    log_event(&AriaEvent::MetadataUpdated {
        mint: *mint,
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    });
}

/// Log tokens burn event
pub fn log_tokens_burned(mint: &Pubkey, source: &Pubkey, amount: u64) {
    log_event(&AriaEvent::TokensBurned {
        mint: *mint,
        source: *source,
        amount,
    });
}

/// Log tokens lock event
pub fn log_tokens_locked(lock_account: &Pubkey, owner: &Pubkey, amount: u64, unlock_time: u64) {
    log_event(&AriaEvent::TokensLocked {
        lock_account: *lock_account,
        owner: *owner,
        amount,
        unlock_time,
    });
}

/// Log tokens unlock event
pub fn log_tokens_unlocked(lock_account: &Pubkey, owner: &Pubkey, destination: &Pubkey, amount: u64) {
    log_event(&AriaEvent::TokensUnlocked {
        lock_account: *lock_account,
        owner: *owner,
        destination: *destination,
        amount,
    });
}

/// Log authority added event
pub fn log_authority_added(authority: &Pubkey, role: u8, admin: &Pubkey) {
    log_event(&AriaEvent::AuthorityAdded {
        authority: *authority,
        role,
        admin: *admin,
    });
}

/// Log authority removed event
pub fn log_authority_removed(authority: &Pubkey, role: u8, admin: &Pubkey) {
    log_event(&AriaEvent::AuthorityRemoved {
        authority: *authority,
        role,
        admin: *admin,
    });
}
