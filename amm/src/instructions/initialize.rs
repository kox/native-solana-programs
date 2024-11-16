use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

/// # Initialize
///
/// -- Data scheme --
/// > Seed: u16
/// > Authority: Flag<Pubkey>
/// > MintX: Pubkey
/// > MintY: Pubkey
/// > MintLP: Pubkey
/// > VaultX: Pubkey
/// > VaultY: Pubkey
/// > Fee: u16
/// > AuthorityBump: u8
///
/// -- Instruction Logic --
///
/// -- Client Side Logic --
///
/// -- Account Optimization Logic --
///
/// -- Checks --
///
pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [config] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(config.is_signer());

    // Populate Config
    unsafe {
        *(config.borrow_mut_data_unchecked().as_ptr() as *mut &[u8]) = data;
    };

    Ok(())
}
