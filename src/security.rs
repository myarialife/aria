use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Security role structure
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum Role {
    /// Admin with full control
    Admin,
    /// Minter can mint new tokens
    Minter,
    /// Freezer can freeze accounts
    Freezer,
    /// Burner can burn tokens
    Burner,
}

/// Authority data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Authority {
    /// Public key of the authorized account
    pub key: Pubkey,
    /// Role assigned to this authority
    pub role: Role,
    /// Whether this authority is active
    pub is_active: bool,
}

/// Authority list data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AuthorityList {
    /// Primary admin that can never be removed
    pub primary_admin: Pubkey,
    /// List of authorities
    pub authorities: Vec<Authority>,
}

impl AuthorityList {
    pub fn new(primary_admin: Pubkey) -> Self {
        let mut authorities = Vec::new();
        authorities.push(Authority {
            key: primary_admin,
            role: Role::Admin,
            is_active: true,
        });
        
        Self {
            primary_admin,
            authorities,
        }
    }

    pub fn has_role(&self, key: &Pubkey, role: Role) -> bool {
        // Primary admin always has all roles
        if key == &self.primary_admin {
            return true;
        }
        
        // Check if key has the specific role
        self.authorities.iter().any(|auth| 
            &auth.key == key && auth.role == role && auth.is_active
        )
    }

    pub fn add_authority(&mut self, key: Pubkey, role: Role) {
        // Don't add duplicates
        if self.authorities.iter().any(|auth| auth.key == key && auth.role == role) {
            return;
        }
        
        self.authorities.push(Authority {
            key,
            role,
            is_active: true,
        });
    }

    pub fn remove_authority(&mut self, key: &Pubkey, role: Role) -> Result<(), ProgramError> {
        // Cannot remove primary admin
        if key == &self.primary_admin && role == Role::Admin {
            return Err(ProgramError::InvalidArgument);
        }
        
        // Find and remove the authority
        if let Some(index) = self.authorities.iter().position(|auth| 
            &auth.key == key && auth.role == role && auth.is_active
        ) {
            self.authorities[index].is_active = false;
            Ok(())
        } else {
            Err(ProgramError::InvalidArgument)
        }
    }
}

/// Initialize the authority list
pub fn initialize_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if authority account is owned by our program
    if authority_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Create new authority list with the signer as primary admin
    let authority_list = AuthorityList::new(*authority.key);

    // Serialize and save the authority list
    authority_list.serialize(&mut *authority_account.data.borrow_mut())?;

    msg!("Authority initialized with primary admin: {}", authority.key);
    
    Ok(())
}

/// Add a new authority
pub fn add_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_authority: Pubkey,
    role: Role,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let admin = next_account_info(account_info_iter)?;

    // Check if admin is a signer
    if !admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if authority account is owned by our program
    if authority_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize authority list
    let mut authority_list = AuthorityList::try_from_slice(&authority_account.data.borrow())?;

    // Verify admin has Admin role
    if !authority_list.has_role(admin.key, Role::Admin) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Add new authority
    authority_list.add_authority(new_authority, role);

    // Serialize and save updated authority list
    authority_list.serialize(&mut *authority_account.data.borrow_mut())?;

    msg!("Added new authority: {} with role: {:?}", new_authority, role);
    
    Ok(())
}

/// Check if an account has a specific role
pub fn check_authority(
    accounts: &[AccountInfo],
    role: Role,
) -> Result<bool, ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let authority_account = next_account_info(account_info_iter)?;
    let authority_to_check = next_account_info(account_info_iter)?;

    // Deserialize authority list
    let authority_list = AuthorityList::try_from_slice(&authority_account.data.borrow())?;

    // Check if the authority has the specified role
    Ok(authority_list.has_role(authority_to_check.key, role))
} 