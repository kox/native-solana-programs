use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [marketplace] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        let marketplace_account = marketplace.borrow_mut_data_unchecked().as_mut_ptr();

        *(marketplace_account.add(0) as *mut Pubkey) = *(data.as_ptr() as *const Pubkey); // maker
        *(marketplace_account.add(32) as *mut u64) = *(data.as_ptr() as *const u64); // fee
        *(marketplace_account.add(32) as *mut u8) = *(data.as_ptr() as *const u8); // bump
        *(marketplace_account.add(32) as *mut u8) = *(data.as_ptr() as *const u8);
        // treasury_bump
    }

    Ok(())
}
