use crate::state::Config;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

/// # Lock
///
/// -- Instruction Logic --
///
/// -- Client Side Logic --
///
/// -- Account Optimization Logic --
///
/// -- Checks --
///
pub fn lock(accounts: &[AccountInfo]) -> ProgramResult {
    let [authority, config] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(authority.is_signer());

    let config_account = Config::from_account_info(config);

    match config_account.get_status() {
        0 => {
            assert_eq!(authority.key(), &config_account.update_authority());
            unsafe { *(config.borrow_mut_data_unchecked().as_mut_ptr() as *mut u8) = 1 };
        }
        1 => {
            assert_eq!(authority.key(), &config_account.update_authority());
            unsafe { *(config.borrow_mut_data_unchecked().as_mut_ptr() as *mut u8) = 0 };
        }
        _ => return Err(ProgramError::InvalidAccountData),
    }

    Ok(())
}
