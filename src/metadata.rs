use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

/// Metadata for the ARIA token
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AriaMetadata {
    /// Name of the token
    pub name: String,
    /// Symbol of the token
    pub symbol: String,
    /// URI for the token logo and additional information
    pub uri: String,
    /// Version of the metadata
    pub version: u8,
}

impl AriaMetadata {
    pub fn new(name: String, symbol: String, uri: String) -> Self {
        Self {
            name,
            symbol,
            uri,
            version: 1,
        }
    }

    pub fn default() -> Self {
        Self {
            name: "ARIA Token".to_string(),
            symbol: "ARI".to_string(),
            uri: "https://ariatoken.io/metadata.json".to_string(),
            version: 1,
        }
    }
}

/// Initialize token metadata
pub fn initialize_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
    uri: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let metadata_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let rent_account = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if metadata account is owned by our program
    if metadata_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if the account has enough lamports to be rent-exempt
    let rent = Rent::from_account_info(rent_account)?;
    if !rent.is_exempt(metadata_account.lamports(), metadata_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Create and serialize the metadata
    let metadata = AriaMetadata::new(name, symbol, uri);
    metadata.serialize(&mut *metadata_account.data.borrow_mut())?;

    Ok(())
}

/// Update token metadata (only by authority)
pub fn update_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let metadata_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if metadata account is owned by our program
    if metadata_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the current metadata
    let mut metadata = AriaMetadata::try_from_slice(&metadata_account.data.borrow())?;

    // Update fields if provided
    if let Some(name) = name {
        metadata.name = name;
    }
    if let Some(symbol) = symbol {
        metadata.symbol = symbol;
    }
    if let Some(uri) = uri {
        metadata.uri = uri;
    }

    // Increment version
    metadata.version += 1;

    // Serialize the updated metadata
    metadata.serialize(&mut *metadata_account.data.borrow_mut())?;

    Ok(())
} 