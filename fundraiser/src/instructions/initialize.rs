use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

pub fn initialize(
    accounts: &[AccountInfo], 
    data: &[u8]
) -> ProgramResult {
    let [maker, fundraiser, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    assert!(fundraiser.is_signer());

    // Copy maker key
    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key() };

    // Copy everything after maker
    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8; 49]) = *(data.as_ptr() as *const [u8; 49])};

    Ok(())
}