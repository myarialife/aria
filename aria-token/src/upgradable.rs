use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

use crate::error::{AriaTokenError, check_account_owner, check_signer, check_rent_exempt};
use crate::security::authorize;

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

/// Initialize the program version
pub fn initialize_version(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(version_account, program_id)?;
    check_signer(authority)?;
    
    if !version_account.data_is_empty() {
        return Err(AriaTokenError::VersionMismatch.into());
    }
    
    // Create new version with the authority as upgrade authority
    let version = ProgramVersion::new(*authority.key);
    
    // Serialize and store version in account
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Program version initialized to {}", version.version_string());
    
    Ok(())
}

/// Schedule an upgrade to a new program id
pub fn schedule_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_program_id: Pubkey,
    upgrade_delay: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(version_account, program_id)?;
    check_signer(authority)?;
    
    // Check if the signer is the upgrade authority
    let mut version = ProgramVersion::deserialize(&mut &version_account.data.borrow()[..])?;
    
    // Verify authority
    authorize(authority, authority_account, 0)?; // Admin role = 0
    
    // Check if an upgrade is already in progress
    if version.pending_upgrade.is_some() {
        return Err(AriaTokenError::UpgradeInProgress.into());
    }
    
    // Get current time from clock sysvar
    let clock = Clock::from_account_info(clock_sysvar)?;
    let current_time = clock.unix_timestamp as u64;
    
    // Schedule the upgrade
    version.pending_upgrade = Some(new_program_id);
    version.upgrade_time = Some(current_time + upgrade_delay);
    
    // Save updated version information
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Upgrade scheduled to program {} at timestamp {}", 
        new_program_id.to_string(), 
        current_time + upgrade_delay
    );
    
    Ok(())
}

/// Finalize an upgrade that has been scheduled
pub fn finalize_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(version_account, program_id)?;
    check_signer(authority)?;
    
    // Load version account data
    let mut version = ProgramVersion::deserialize(&mut &version_account.data.borrow()[..])?;
    
    // Check if the signer is the upgrade authority
    if *authority.key != version.upgrade_authority {
        return Err(AriaTokenError::IncorrectAuthority.into());
    }
    
    // Check if an upgrade is scheduled
    let pending_upgrade = version.pending_upgrade.ok_or(AriaTokenError::NoUpgradeScheduled)?;
    let upgrade_time = version.upgrade_time.ok_or(AriaTokenError::NoUpgradeScheduled)?;
    
    // Check if upgrade time has been reached
    let clock = Clock::from_account_info(clock_sysvar)?;
    let current_time = clock.unix_timestamp as u64;
    
    if current_time < upgrade_time {
        return Err(AriaTokenError::UpgradeTimeNotReached.into());
    }
    
    // Update version information
    version.increment_minor(); // Default is to increment minor version
    version.pending_upgrade = None;
    version.upgrade_time = None;
    
    // Save updated version information
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Upgrade finalized. New version: {}", version.version_string());
    
    Ok(())
}

/// Cancel a scheduled upgrade
pub fn cancel_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let version_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    
    // Check accounts
    check_account_owner(version_account, program_id)?;
    check_signer(authority)?;
    
    // Load version account data
    let mut version = ProgramVersion::deserialize(&mut &version_account.data.borrow()[..])?;
    
    // Check if the signer is the upgrade authority
    if *authority.key != version.upgrade_authority {
        return Err(AriaTokenError::IncorrectAuthority.into());
    }
    
    // Check if an upgrade is scheduled
    if version.pending_upgrade.is_none() {
        return Err(AriaTokenError::NoUpgradeScheduled.into());
    }
    
    // Cancel the scheduled upgrade
    version.pending_upgrade = None;
    version.upgrade_time = None;
    
    // Save updated version information
    version.serialize(&mut *version_account.data.borrow_mut())?;
    
    msg!("Scheduled upgrade has been cancelled");
    
    Ok(())
}
