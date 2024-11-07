use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, sysvars::{clock::Clock, Sysvar}, ProgramResult};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};
use solana_nostd_sha256::hashv;

use crate::{Fundraiser, ID};

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

/// Checker
/// Instruction signed by maker to retrieve the funds from the vault and send them to the maker token account
///
/// Accounts:
/// > maker                 - signer
/// > maker_ta              - Token account of maker where the tokens should be sent
/// > fundraiser            - PDA containg all relevant data (in this case we need the bump)
/// > vault                 - TA storing the fundraise tokens (owned by program)
/// > Token Program       - Program (we should use it for the Transfer CPI)
///
/// Checks:
/// > It shoud have expired and it should have reach the fundarise goal and it should be the maker
///
pub fn checker(
    accounts: &[AccountInfo], 
    data: &[u8]
) -> ProgramResult {
    let [maker, maker_ta, fundraiser, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // It should have ended the time period
    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    assert!(Clock::get()?.slot > fundraiser_account.slot());

    // it should have reach the goal remaining_account == 0
    assert_eq!(fundraiser_account.remaining_amount(), 0);

    assert!(maker.is_signer());
    assert_eq!(&fundraiser_account.maker(), maker.key());

    let bump = [unsafe { *(data.as_ptr() as *const u8) }];

    // The program will need to sign the transaction
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(bump.as_ref())];
    let signers = [Signer::from(&seeds)];

    // Let's generate the vault with the fundraiser and the bump (data)
    let vault_pda = hashv(&[
        fundraiser.key().as_ref(),
        &bump,
        ID.as_ref(),
        PDA_MARKER,
    ]);

    // Let's validate is the correct vault
    assert_eq!(&vault_pda, vault.key().as_ref());
/* 
    // Let's get the amount of tokens funded in the vault
    let vault_account = TokenAccount::from_account_info_unchecked(vault);
    let amount = vault_account.amount();

    // Now we can transfer the tokens doing the CPI
    Transfer {
        from: vault,
        to: maker_ta,
        authority: vault,
        amount,
    }.invoke_signed(&signers)?; */

    // once we transfer the tokens, i guess we would like to get the lamports too :D


    /* let [maker, fundraiser, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    assert!(fundraiser.is_signer());

    // Copy maker key
    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key() };

    // Copy everything after maker
    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8; 49]) = *(data.as_ptr() as *const [u8; 49])};
 */
    Ok(())
}