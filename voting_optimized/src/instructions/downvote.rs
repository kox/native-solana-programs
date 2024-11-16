use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

pub fn downvote(
    accounts: &[AccountInfo], 
) -> ProgramResult {
    let [vote_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    unsafe { 
        // Do i need to do a overflow check? I guess it will fail anyways so we can save CUs no checking it 
        *(vote_account.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) -= 1u64;
    };

    Ok(())
}