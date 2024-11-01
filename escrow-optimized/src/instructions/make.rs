use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};


pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, escrow, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(escrow.is_signer());

    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key();
    }

    unsafe {
        *((escrow.borrow_mut_data_unchecked().as_mut_ptr().add(size_of::<Pubkey>) as *mut Pubkey) as *mut [u8;32])= *data;
    }
    

    Ok(())
}