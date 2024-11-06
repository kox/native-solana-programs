use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

 

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [config] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(config.is_signer()); // hand the account to the program

    unsafe {
        config.borrow_data_unchecked().copy_from_slice(&data);
    }
}