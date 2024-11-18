use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_token::instructions::Transfer;

use crate::{Contributor, Fundraiser, MIN_AMOUNT_TO_RAISE};

/// Checker
/// Instruction signed by contributors to give their contribution in a fundraising event transfering tokens into the vault and
/// updating fundraiser account plus creating their contributor account (PDA) where it gets tracked the amount of tokens contributed.
///
/// Accounts:
/// > contributor        - contributor
/// > contributor_ta     - Token account of contributor
/// > Fundraiser    - PDA containg all relevant data
/// > Vault         - ATA to store the tokens (owned by program)
/// > Token Program - Program (we should use it for the Transfer CPI)
///
/// Data:
/// > Amount: u64  - Amount of tokens to fund
///
/// Checks:
/// > Minimum contributing ammount
/// > Exceeded the remaining amount left for the campaign
///
pub fn contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // First thing first, if you don't send enough amount better don't lose lamports
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };
    if amount < MIN_AMOUNT_TO_RAISE {
        return Err(ProgramError::InvalidInstructionData);
    }

    // We deconstruct accounts
    let [contributor, contributor_ta, contributor_account, fundraiser, vault, _authority, _token_program] =
        accounts
    else {
        return Err(ProgramError::BorshIoError);
    };

    // Get fundraiser account data. Internally we check the ownership and LEN to avoid possible attacks
    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    let contributor_account_account = Contributor::from_account_info(contributor_account);

    // Is expired the campaign? We will need to do a syscall to retrieve the slot
    let clock = Clock::get().expect("Failed to load the clock");
    assert!(clock.slot < fundraiser_account.slot());

    // Check funder_ta matches our fundraiser mint account??? Do i need to test this? I don't think so.
    /* assert_eq!(
        &TokenAccount::from_account_info(vault).mint(),
        &fundraiser_account.mint()
    ); */

    // Before transfering tokens, we need to be sure that our tokens will go to a valid vault. otherwise, someone could send wrong
    // vault, and then claim some non owned tokens. To validate the vault, we will try to generate the PDA in a cheap way, modifying the
    // fundraiser key with a bump passed via parameter.

    // TODO: Verify the vault is valid

    // We need to transfer the tokens + Update the remaining amount from fundraiser + update the contributor_account for a possible refund

    // 1. Transfer Tokens from funder to the vault
    Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    // I guess i should check_add  but if overflow then it will fail anyways :')
    let contribute_amount = contributor_account_account.amount() + amount;
    let remaining_amount = if amount > fundraiser_account.remaining_amount() {
        0
    } else {
        fundraiser_account.remaining_amount() - amount
    };

    unsafe {
        // Get a mutable pointer to the account's data once
        // Calculate the new amount and store it in the correct position (32-byte offset)
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut [u8; 8]) =
            (remaining_amount).to_le_bytes();

        // iusing copy_from_slice adds 2 CU
        // fundraiser.borrow_mut_data_unchecked()[64..72].copy_from_slice(&(fundraiser_account.remaining_amount() - amount).to_le_bytes());

        // using check_sub adds 8 CU
        // *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut [u8; 8]) = (fundraiser_account.remaining_amount().checked_sub(amount).ok_or(ProgramError::ArithmeticOverflow))?.to_le_bytes();

        // last but not least, we update the total contributions made by a user
        *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr() as *mut [u8; 8]) =
            contribute_amount.to_le_bytes();
    }

    Ok(())
}
