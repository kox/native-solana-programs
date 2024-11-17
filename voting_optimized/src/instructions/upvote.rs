use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

pub fn upvote(
    accounts: &[AccountInfo], 
) -> ProgramResult {
    let [vote_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    unsafe { 
        *(vote_account.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) += 1u64;
    };

    Ok(())
}