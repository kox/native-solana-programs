use std::borrow::BorrowMut;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, msg, program::invoke_signed, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey};
use spl_token::instruction::transfer_checked;

use crate::{error::EscrowError, Escrow};

#[inline]
pub fn process_refund_instruction(accounts: &[AccountInfo<'_>], _instruction_data: &[u8]) -> Result<(), ProgramError> {

    msg!("Loading Accounts");

    let [maker, escrow, mint, vault, maker_ata, token_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let escrow_data = Escrow::try_from_slice(&escrow.try_borrow_mut_data()?)?;
    let escrow_pda = Pubkey::find_program_address(&[b"escrow", maker.key.as_ref(), escrow_data.seed.to_le_bytes().as_ref()], &crate::ID);

    if !maker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if escrow.key.ne(&escrow_pda.0) {
        return Err(EscrowError::EscrowAccountMismatch.into());
    }
    
    let vault_data = spl_token::state::Account::unpack(&vault.try_borrow_mut_data()?)?;
    if vault_data.mint.ne(mint.key) && vault_data.owner.ne(&escrow_pda.0) {
        return Err(EscrowError::EscrowAccountMismatch.into());
    }

    let maker_ata_data = spl_token::state::Account::unpack(&maker_ata.try_borrow_mut_data()?)?;
    if maker_ata_data.mint.ne(mint.key) && maker_ata_data.owner.ne(maker.key) {
        return Err(EscrowError::EscrowAccountMismatch.into());
    }

    let decimals = spl_token::state::Mint::unpack(&mint.try_borrow_mut_data()?)?.decimals;
    let transfer_ix = transfer_checked(token_program.key, vault.key, mint.key, maker_ata.key, escrow.key, &[], vault_data.amount, decimals)?;
    invoke_signed(
        &transfer_ix, 
        &[vault.clone(), mint.clone(), maker_ata.clone(), escrow.clone()],
        &[&[b"escrow", maker.key.as_ref(), escrow_data.seed.to_le_bytes().as_ref(), &[escrow_pda.1]]]
    )?;

    let escrow_balance = escrow.lamports();
    *escrow.lamports().borrow_mut() = 0;
    *maker.lamports().borrow_mut() += escrow_balance;

    Ok(())
}