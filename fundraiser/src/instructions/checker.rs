use pinocchio::{account_info::AccountInfo, program_error::ProgramError, sysvars::{clock::Clock, Sysvar}, ProgramResult};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{ Fundraiser, MIN_AMOUNT_TO_RAISE };

/// Checker
/// Instruction signed by funders to contribute in the fundraising transfering tokens into the vault and
/// updating fundraiser account plus creating a new Contributor PDA where it gets recorded the amount.
/// 
/// Accounts: 
/// > Funder        - signer
/// > Funder_TA     - Token account of Funder
/// > Fundraiser    - PDA containg all relevant data
/// > Vault         - ATA to store the tokens (owned by authority)
/// > Token Program - Program 
/// 
/// Data:
/// > Amount: u64  - Amount of tokens to fund
/// 
/// Checks:
/// Minimum contributing ammount 
pub fn checker(
    accounts: &[AccountInfo], 
    data: &[u8]
) -> ProgramResult {
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };

    if amount < MIN_AMOUNT_TO_RAISE {
        return Err(ProgramError::InvalidInstructionData);
    }

    let [
        funder, 
        funder_ta, 
        fundraiser,
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
        from: funder_ta,
        to: vault,
        authority: funder,
        amount,
    }.invoke()?; 

    /*
    // Update Fundraiser remaining. Checking overflow will crash so i don't think it's needed... it could be done on the client 
    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut u64) = fundraiser_account.remaining_amount() - amount;
    } */


    Ok(())
}