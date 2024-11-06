use pinocchio::{account_info::AccountInfo, program_error::ProgramError, sysvars::{clock::Clock, Sysvar}, ProgramResult};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{ state::fundraiser, Fundraiser, MIN_AMOUNT_TO_RAISE };

/// Checker
/// Instruction signed by contributors to give their contribution in a fundraising event transfering tokens into the vault and
/// updating fundraiser account plus creating their contributor account (PDA) where it gets tracked the amount of tokens contributed.
/// 
/// Accounts: 
/// > contributor        - contributor
/// > contributor_ta     - Token account of contributor
/// > Fundraiser    - PDA containg all relevant data
/// > Vault         - ATA to store the tokens (owned by authority)
/// > Token Program - Program (we should use it for the Transfer CPI)
/// 
/// Data:
/// > Amount: u64  - Amount of tokens to fund
/// 
/// Checks:
/// > Minimum contributing ammount 
/// > Exceeded the remaining amount left for the campaign
/// 
pub fn contribute(
    accounts: &[AccountInfo], 
    data: &[u8]
) -> ProgramResult {
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };

    if amount < MIN_AMOUNT_TO_RAISE {
        return Err(ProgramError::InvalidInstructionData);
    }

    let [
        contributor, 
        contributor_ta, 
        fundraiser,
        mint,
        vault,
        _authority,
        _token_program 
    ] = accounts else {
        return Err(ProgramError::BorshIoError);
    };

    // Get fundraiser account data
    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // Amount needs to be lower or equal than remaining_amount
    if amount < MIN_AMOUNT_TO_RAISE {
        return Err(ProgramError::InvalidInstructionData);
    }

    let clock = Clock::get().expect("Failed to load the clock");

    // Is expired the campaign?
    assert!(clock.slot < fundraiser_account.slot());
    
     
    // Check funder_ta matches our fundraiser mint account
    assert_eq!(
        &TokenAccount::from_account_info(vault).mint(),
        &fundraiser_account.mint()
    );

    // We need to transfer the tokens + create the PDA with the amount + Update the remaining amount from fundraiser

    // 1. Transfer Tokens from funder to the vault
    Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }.invoke()?;

    unsafe {
        // Get a mutable pointer to the account's data once
        // Calculate the new amount and store it in the correct position (32-byte offset)
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut [u8; 8]) = (fundraiser_account.remaining_amount() - amount).to_le_bytes();

        // iusing copy_from_slice adds 2 CU
        // fundraiser.borrow_mut_data_unchecked()[64..72].copy_from_slice(&(fundraiser_account.remaining_amount() - amount).to_le_bytes());

        // using check_sub adds 8 CU
        // *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut [u8; 8]) = (fundraiser_account.remaining_amount().checked_sub(amount).ok_or(ProgramError::ArithmeticOverflow))?.to_le_bytes();
    }

    Ok(())
}