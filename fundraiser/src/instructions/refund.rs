use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::state::{contributor, fundraiser};

/// Refund
/// Instruction signed by contributors to give their retrieve their contribution and close that PDA account. As the PDA belongs to the program,
/// it's needed to by signed on behalf of the program. It should update fundraiser to update the remaining amount needed to raise. We are refunding all
/// contributions (TODO: pass amount to refund only a part of it)
///
/// Accounts:
/// > contributor        - contributor
/// > contributor_ta     - Token account of contributor where the tokens should be sent
/// > Fundraiser         - PDA containg all relevant data (in this case we need the bump)
/// > Vault              - ATA storing the contributor tokens (owned by authority)
/// > Token Program - Program (we should use it for the Transfer CPI)
///
/// Checks:
/// > It shoud not have expired, otherwise someone could get the tokens after
///
pub fn refund(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, contributor_ta, contributor_account, fundraiser, vault, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // I don't think i need to check if it's signer contributor
    // assert!(contributor.is_signer());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);


    /// Make sure that the period has elapsed and we didn't reach the goal
    /* let clock = Clock::get().expect("Failed to load the clock");

    // Is expired the campaign? */
    assert!(Clock::get().expect("Failed to load the clock") < fundraiser_account.slot());

    Ok(())
}
