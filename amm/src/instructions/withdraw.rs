

use std::sync::Arc;

use mollusk_svm::result::ProgramResult;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};
use pinocchio_token::{instructions::MintTo, state::TokenAccount};

use crate::state::Config;

pub fn withdraw(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        provider,
        config,
        authority,
        provider_x,
        provider_y,
        provider_lp,
        vault_x,
        vault_y,
        mint_lp,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let config_account = Config::from_account_info_unchecked(config);

    assert_eq!(&config_account.vault_x(), vault_x.key());
    assert_eq!(&config_account.vault_y(), vault_y.key());

    // assert_eq!(&config_account.mint_lp(), mint_lp.key());

    let (amount, max_x, max_y) = unsafe {
        *(data.as_ptr() as *const (u64, u64, u64))
    };

    let (supply_lp, supply_x, supply_y) = unsafe { (
            Mint::from_account_info_unchecked(mint_lp)?.supply(),
            TokenAccount::from_account_info_unchecked(vault_x)?.amount(),
            TokenAccount::from_account_info_unchecked(vault_y)?.amount(),
        )
    };

    let precision = 1_000_000u128;
    let ratio = (u128::from(supply_lp)
        .checked_add(u128::from(amount))
        .ok_or(ProgramError::ArithmeticOverflow)?)
        .checked_mul(precision).ok_or(ProgramError::ArithmeticOverflow)?;

    let (deposit_x_amount, deposit_y_amount) = if supply_lp == 0 {

    } else {
        (
        u64::from((u128::from(supply_x)
            .checked_mul(ratio)
            .ok_or(ProgramError::ArithmeticOverflow)?)
            .checked_div(precision)
            .ok_or(ProgramError::ArithmeticOverflow)?),
    
        u64::from((u128::from(supply_y)
            .checked_mul(ratio)
            .ok_or(ProgramError::ArithmeticOverflow)?)
            .checked_div(precision)
            .ok_or(ProgramError::ArithmeticOverflow)?)
        )
    };


    assert!(deposit_x_amount.le(&max_x));
    assert!(deposit_y_amount.le(&max_x));

    Transfer {
        from: provider_x,
        to: vault_x,
        authority: provider,
        amount: deposit_x_amount,
    }.invoke()?;

    Transfer {
        from: provider_y,
        to: vault_y,
        authority: provider,
        amount: deposit_y_amount,
    }.invoke()?;

    let binding = config_account.bump();
    let seeds = [
        Seed::from(config.key().as_ref(),
        Seed::from(&binding),
    )];

    MintTo {
        mint: mint_lp,
        token: provider_lp,
        mint_authority: authority,
        amount,
    }.invoke_signed(signers);

    /* let multiplier = (supply_lp + amount) / supply_lp;
    let deposit_x_amount  = multiplier * supply_x - supply_x;
    let deposit_y_amount  = multiplier * supply_y - supply_y; */

    /* Another way
    supply_x * 10000 /supply_lp * amount / 1000;
    supply_y * 10000 /supply_lp * amount / 1000; 
    */
     

}