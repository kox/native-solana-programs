use constant_product_curve::xy_deposit_amounts_from_l;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_token::{
    instructions::{MintTo, Transfer},
    state::{Mint, TokenAccount},
};

use crate::state::Config;

/// # Deposit
///
/// -- Data scheme --
/// Amount: u64
/// MaxX: u64
/// MaxY: u64
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
pub fn deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, authority, mint_lp, user_x, user_y, user_lp, vault_x, vault_y, config, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Deserialize Data
    let (amount, max_x, max_y, expiration) = unsafe {
        let [amount, max_x, max_y, expiration] = *(data.as_ptr() as *const [u64; 4]);
        (amount, max_x, max_y, expiration as i64)
    };

    // Checks
    let config_account = Config::from_account_info(config);
    assert_ne!(config_account.get_status(), 1);
    assert_eq!(mint_lp.key(), &config_account.mint_lp());
    assert_eq!(vault_x.key(), &config_account.vault_x());
    assert_eq!(vault_y.key(), &config_account.vault_y());
    assert!(expiration < Clock::get()?.unix_timestamp);

    // Calculate the amount of LP tokens to mint and the amount of tokens to deposit
    let supply = unsafe { Mint::from_account_info_unchecked(mint_lp)?.supply() };
    let vault_x_amount = unsafe { TokenAccount::from_account_info_unchecked(vault_x)?.amount() };
    let vault_y_amount = unsafe { TokenAccount::from_account_info_unchecked(vault_y)?.amount() };

    let (x, y) = match supply == 0 && vault_x_amount == 0 && vault_y_amount == 0 {
        true => (max_x, max_y),
        false => xy_deposit_amounts_from_l(
            vault_x_amount,
            vault_y_amount,
            supply,
            amount,
            1_000_000_000,
        )
        .map_err(|_| ProgramError::ArithmeticOverflow)?,
    };

    assert!(x <= max_x);
    assert!(y <= max_y);

    // Deposit Tokens
    Transfer {
        from: user_x,
        to: vault_x,
        authority: user,
        amount: x,
    }
    .invoke()?;

    Transfer {
        from: user_y,
        to: vault_y,
        authority: user,
        amount: y,
    }
    .invoke()?;

    // Derive the signer
    let binding = [config_account.authority_bump()];
    let seeds = [Seed::from(config.key().as_ref()), Seed::from(&binding)];
    let signer = [Signer::from(&seeds)];

    // Mint LP Tokens
    MintTo {
        mint: mint_lp,
        token: user_lp,
        mint_authority: authority,
        amount,
    }
    .invoke_signed(&signer)?;

    Ok(())
}
