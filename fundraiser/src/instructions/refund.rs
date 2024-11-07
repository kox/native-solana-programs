use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, sysvars::{clock::Clock, Sysvar}, ProgramResult};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{ Contributor, Fundraiser };

/// Refund
/// Instruction signed by contributors to give their retrieve their contribution and close that PDA account. As the PDA belongs to the program,
/// it's needed to by signed on behalf of the program. It should update fundraiser to update the remaining amount needed to raise. We are refunding all
/// contributions (TODO: pass amount to refund only a part of it)
///
/// Accounts:
/// > contributor         - contributor
/// > contributor_ta      - Token account of contributor where the tokens should be sent
/// > contributor_account - PDA tracking the contributor's support 
/// > fundraiser          - PDA containg all relevant data (in this case we need the bump)
/// > vault               - ATA storing the contributor tokens (owned by authority)
/// > Token Program       - Program (we should use it for the Transfer CPI)
///
/// Checks:
/// > It shoud not have expired, otherwise someone could get the tokens after
///
pub fn refund(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        contributor, 
        contributor_ta, 
        contributor_account, 
        fundraiser, 
        vault, 
        token_program
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(contributor.is_signer());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // Is expired the campaign? 
    let clock = Clock::get()?.slot;
    // assert!(fundraiser_account.slot() == 2); 
    

    assert!(Clock::get()?.slot > fundraiser_account.slot());

    // Make sure that we didnt reach the goal
    let vault_account = unsafe { TokenAccount::from_account_info_unchecked(vault) };
    assert!(fundraiser_account.remaining_amount() > 0u64);
 
    assert_eq!(fundraiser_account.mint(), vault_account.mint());
    
    let bump = [unsafe { *(data.as_ptr() as *const u8) }];
    
     
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];
 
    // We transfer contributor amount to its owner
    Transfer {
        from: vault,
        to: contributor_ta,
        authority: vault,
        amount: Contributor::from_account_info(contributor_account).amount(),
    }.invoke_signed(&signers)?;

    // closing contributor account
    unsafe {
        *contributor.borrow_mut_lamports_unchecked() += *contributor_account.borrow_lamports_unchecked();
        *contributor_account.borrow_mut_lamports_unchecked() = 0;

        contributor_account.assign(&Pubkey::default());

        *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;

        // at least 50 more CUs
        // contributor_account.realloc(0, false);
    }

    Ok(())
}
