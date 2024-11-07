use pinocchio::{account_info::AccountInfo, program_error::ProgramError, sysvars::clock::Clock};
use pinocchio_token::state::{Token, TokenAccount};

pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        buyer, 
        buyer_from,
        buyer_to, 
        authority,
        buyer_ta_to,
        vault_x,
        vault_y,
        config,
        token_program,
        ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let in_amount = unsafe { *(data.as_ptr() as *const u64) };
    let min_out_amount = unsafe { *(data.as_ptr().add(8) as *const u64) };
    
    /* // time-based execution 
    let expiration = unsafe { *(data.as_ptr().add(16) as *const i64) };
    assert!(expiration < Clock::get()?.unix_timestamp); */
    // Another way to time a swap is using a blockhash valid for the next 20/30 seconds

    let config_account = Config::from_account_info(config);

    let is_x = vault_from.key().eq(config_account.vault_x());

    let is_x = vault_from.key().eq(&config_account.vault_x());

    if is_x {
        assert!(vault_to.key().eq(&config_account.vault_y()));
    } else {
        assert!(vault_from.key().eq(&config_account.vault_y));
        assert!(vault_to.key().eq(&config_account.vault_x));
    }

    let vault_from_amount = unsafe {
        TokenAccount::from_account_info_unchecked(vault_from)?.amount()
    };

    let vault_to_amount = unsafe {
        TokenAccount::from_account_info_unchecked(vault_to)?.amount()  
    };

    // curve logic to ge the correct amount out
    // K = XY
    let k = u128::from(vault_from_amount).cheked_mul(u128::from(vault_to_amount)).ok_or(ProgramError::ArithmeticOverflow);
    // X2 = k + X(n)
    let new_vault_from_amount = u128::from(vault_from_amount).checked_add(u128::from(in_amount)).ok_or(ProgramError::ArithmeticOverflow);
    // Y2 = K/X2
    let new_vault_to_amount = k.checked_div(new_vault_from_amount).ok_or(ProgramError::ArithmeticOverflow);
    // Y out = Y2 - Y
    let delta_vault_to_amount = u64::try_from(new_vault_to_amount.checked_sub(u128::from(vault_to_amount)).ok_or(ProgramError::ArithmeticOverflow)?).map_err(|_| ProgramError::ArithmeticOverflow)?;

    // deduct fees
    let fees = delta_vault_to_amount
        .checked_mul(config_amount.fee as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let delta_vault_to_amount_minus_fees = delta_vault_to_amount - fees;

    assert!(delta_vault_to_amount >= min_out_amount);

    Transfer {
        from: buyer_from,
        to: vault_from,
        authority,
        amount: in_amount,
    };

    let seeds = [
        Seed::from(config.key().as_ref())
    ]

    Transfer {
        from: buyer_from,
        to: vault_from,
        authority,
        amount: delta_vault_to_amount,
    };


    assert!(config.is_signer()); // hand the account to the program

    unsafe {
        config.borrow_data_unchecked().copy_from_slice(&data);
    }
}