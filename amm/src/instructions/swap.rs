use constant_product_curve::{
    delta_x_from_y_swap_amount_with_fee, delta_y_from_x_swap_amount_with_fee,
};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::Config;

/// # Swap
///
/// -- Data scheme --
/// Amount: u64
/// MinAmount: u64
/// Expiration: i64
///
/// -- Instruction Logic --
///
/// -- Client Side Logic --
///
/// -- Account Optimization Logic --
///
/// -- Checks --
///
pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, authority, user_x, user_y, vault_from, vault_to, config, _token_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Deserialize Data
    let (amount, min, expiration) = unsafe {
        let [amount, min, expiration] = *(data.as_ptr() as *const [u64; 3]);
        (amount, min, expiration as i64)
    };

    // Checks
    let config_account = Config::from_account_info(config);
    assert_ne!(config_account.get_status(), 1);
    assert!(expiration < Clock::get()?.unix_timestamp);

    let is_x = vault_from.key().eq(&config_account.vault_x());
    if is_x {
        assert_eq!(vault_to.key(), &config_account.vault_y());
    } else {
        assert_eq!(vault_to.key(), &config_account.vault_x());
        assert_eq!(vault_from.key(), &config_account.vault_y());
    }

    // Calculate the amount of LP tokens to mint and the amount of tokens to deposit
    let vault_from_amount =
        unsafe { TokenAccount::from_account_info_unchecked(vault_from)?.amount() };
    let vault_to_amount = unsafe { TokenAccount::from_account_info_unchecked(vault_to)?.amount() };

    // Determine swap direction and fee calculation
    let (amount_out, _) = if is_x {
        delta_y_from_x_swap_amount_with_fee(
            vault_from_amount,
            vault_to_amount,
            amount,
            config_account.fee(),
        )
    } else {
        delta_x_from_y_swap_amount_with_fee(
            vault_from_amount,
            vault_to_amount,
            amount,
            config_account.fee(),
        )
    }
    .map_err(|_| ProgramError::ArithmeticOverflow)?;

    // Slippage check
    assert!(amount_out >= min);

    // Derive the signer
    let binding = [config_account.authority_bump()];
    let seeds = [Seed::from(config.key().as_ref()), Seed::from(&binding)];
    let signer = [Signer::from(&seeds)];

    // Deposit Tokens
    Transfer {
        from: if is_x { user_x } else { user_y },
        to: vault_from,
        authority: user,
        amount,
    }
    .invoke()?;

    Transfer {
        from: vault_to,
        to: if is_x { user_y } else { user_x },
        authority,
        amount: amount_out,
    }
    .invoke_signed(&signer)?;

    Ok(())
}
