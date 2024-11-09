use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use crate::{Fundraiser, ID, PDA_MARKER};
use pinocchio_token::{
    instructions::{CloseAccount, Transfer},
    state::TokenAccount,
};

/// Checker
/// Instruction signed by maker to retrieve the funds from the vault and send them to the maker token account
///
/// Accounts:
/// > maker                 - signer
/// > maker_ta              - Token account of maker where the tokens should be sent
/// > fundraiser            - PDA containg all relevant data (in this case we need the bump)
/// > vault                 - TA storing the fundraise tokens (owned by program)
/// > authority             - PDA account to sign off instructions on behalf of the program
/// > Token Program       - Program (we should use it for the Transfer CPI)
///
/// Checks:
/// > It shoud have expired and it should have reach the fundarise goal and it should be the maker
///
pub fn checker(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [
        maker, 
        maker_ta, 
        fundraiser, 
        vault, 
        authority,
        _token_program
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // It should have ended the time period
    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    assert!(Clock::get()?.slot > fundraiser_account.slot());

    // it should have reach the goal remaining_account == 0
    assert_eq!(fundraiser_account.remaining_amount(), 0); 

    assert!(maker.is_signer());

    // We verify that person trying to claim the 
    assert_eq!(&fundraiser_account.maker(), maker.key());
 
    // We need to sign on behalf of the program
    let bump_binding = fundraiser_account.bump().to_le_bytes();
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(bump_binding.as_ref())];
    let signers = [Signer::from(&seeds)];

    let vault_amount = unsafe { TokenAccount::from_account_info_unchecked(vault)?.amount() };

    // We transfer contributor amount to its owner
    Transfer {
        from: vault,
        to: maker_ta,
        authority,
        amount: vault_amount,
    }.invoke_signed(&signers)?;

    CloseAccount {
        account: vault,
        destination: maker,
        authority,
    }
    .invoke_signed(&signers)?;

    Ok(())
}
