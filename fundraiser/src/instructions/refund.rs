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
pub fn refund(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [
        contributor, 
        contributor_ta, 
        contributor_account, 
        fundraiser, 
        vault,
        authority,
        _token_program
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(contributor.is_signer());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    // Is expired the campaign? 
    let clock = Clock::get()?.slot;
    assert!(clock > fundraiser_account.slot());

    // Make sure that we didnt reach the goal
    assert!(fundraiser_account.remaining_amount() > 0u64);
    
    let vault_account = unsafe { TokenAccount::from_account_info_unchecked(vault)? };
    
    // Do we need to be sure about this check? what can go wrong?
    assert_eq!(&fundraiser_account.mint(), vault_account.mint());
    
    // We need to sign on behalf of the program
    let bump_binding = fundraiser_account.bump().to_le_bytes();
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(bump_binding.as_ref())];

    // let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(fundraiser_account.bump().to_le_bytes().as_ref())];
    let signers = [Signer::from(&seeds)];
 
    // We transfer contributor amount to its owner
    Transfer {
        from: vault,
        to: contributor_ta,
        authority,
        amount: Contributor::from_account_info(contributor_account).amount(),
    }.invoke_signed(&signers)?;

    // closing contributor account
    unsafe {
        *contributor.borrow_mut_lamports_unchecked() += *contributor_account.borrow_lamports_unchecked();
        *contributor_account.borrow_mut_lamports_unchecked() = 0;

        // Disrepectful compiler way (dean) =>  6506 (123 CU less with ASM)
        based_close(contributor_account.borrow_mut_data_unchecked().as_mut_ptr());

        // Old school for deleting account => 6629 CU
        /* contributor_account.assign(&Pubkey::default());
        *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
 */
        // at least 50 more CUs
        // contributor_account.realloc(0, false);
    }

    Ok(())
}


#[inline(always)]
pub fn based_close(data_ptr: *mut u8) {
    #[cfg(target_os = "solana")]
    unsafe {
        let var = 0u64;
        core::arch::asm!(
            "stxdw [{0}-8], {1}", // data len
            "stxdw [{0}-16], {1}", // lamports
            "stxdw [{0}-24], {1}", // owner[24..32]
            "stxdw [{0}-32], {1}", // owner[16..24]
            "stxdw [{0}-40], {1}", // owner[8..16]
            "stxdw [{0}-48], {1}", // owner[0..8]
            in(reg) data_ptr,
            in(reg) var,
            options(nostack, preserves_flags)
        );
    }
}
