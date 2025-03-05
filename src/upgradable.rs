use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::security::{Role, check_authority};

/// Version information for program upgrades
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramVersion {
    /// Current version number (major.minor.patch)
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    /// Authority allowed to upgrade the program
    pub upgrade_authority: Pubkey,
    /// New program ID if an upgrade is in progress
    pub pending_upgrade: Option<Pubkey>,
    /// Time when the pending upgrade is valid
    pub upgrade_time: Option<u64>,
}

impl ProgramVersion {
    pub fn new(upgrade_authority: Pubkey) -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
            upgrade_authority,
            pending_upgrade: None,
            upgrade_time: None,
        }
    }
    
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    
    pub fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }
    
    pub fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }
    
    pub fn increment_patch(&mut self) {
        self.patch += 1;
    }
}

/// Initialize the program version data
pub fn initialize_version(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check if version account is owned by our program
    if version_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Create new version data
    let version = ProgramVersion::new(*authority.key);
    
    // Serialize and save the version data
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Program version initialized: {}", version.version_string());
    
    Ok(())
}

/// Schedule a program upgrade
pub fn schedule_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_program_id: Pubkey,
    upgrade_delay: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check if authority has admin role
    if !check_authority(&[authority_account.clone(), authority.clone()], Role::Admin)? {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Deserialize version data
    let mut version = ProgramVersion::try_from_slice(&version_account.data.borrow())?;
    
    // Only the upgrade authority can schedule upgrades
    if version.upgrade_authority != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Get current time
    let clock = solana_program::clock::Clock::from_account_info(clock_info)?;
    let current_time = clock.unix_timestamp as u64;
    
    // Set the pending upgrade
    version.pending_upgrade = Some(new_program_id);
    version.upgrade_time = Some(current_time + upgrade_delay);
    
    // Serialize and save the updated version data
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Program upgrade scheduled to {} at timestamp {}", 
        new_program_id, 
        version.upgrade_time.unwrap()
    );
    
    Ok(())
}

/// Finalize a program upgrade
pub fn finalize_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Deserialize version data
    let mut version = ProgramVersion::try_from_slice(&version_account.data.borrow())?;
    
    // Only the upgrade authority can finalize upgrades
    if version.upgrade_authority != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if there is a pending upgrade
    let pending_upgrade = match version.pending_upgrade {
        Some(upgrade) => upgrade,
        None => return Err(ProgramError::InvalidAccountData),
    };
    
    // Check if the upgrade time has passed
    let upgrade_time = match version.upgrade_time {
        Some(time) => time,
        None => return Err(ProgramError::InvalidAccountData),
    };
    
    // Get current time
    let clock = solana_program::clock::Clock::from_account_info(clock_info)?;
    let current_time = clock.unix_timestamp as u64;
    
    if current_time < upgrade_time {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Increment version
    version.increment_minor();
    
    // Clear the pending upgrade
    version.pending_upgrade = None;
    version.upgrade_time = None;
    
    // Serialize and save the updated version data
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Program upgraded to version {}", version.version_string());
    
    Ok(())
}

/// Cancel a pending program upgrade
pub fn cancel_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check if authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Deserialize version data
    let mut version = ProgramVersion::try_from_slice(&version_account.data.borrow())?;
    
    // Only the upgrade authority can cancel upgrades
    if version.upgrade_authority != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check if there is a pending upgrade
    if version.pending_upgrade.is_none() {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Clear the pending upgrade
    version.pending_upgrade = None;
    version.upgrade_time = None;
    
    // Serialize and save the updated version data
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Program upgrade cancelled");
    
    Ok(())
}