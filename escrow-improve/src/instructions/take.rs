use std::borrow::BorrowMut;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, program::{invoke, invoke_signed}, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey};

use crate::Escrow;

#[inline]
pub fn process_take_instruction(accounts: &[AccountInfo<'_>], _instruction_data: &[u8]) -> Result<(), ProgramError> {
    let [maker, taker, escrow, mint_a, mint_b, maker_ata, taker_ata_a, taker_ata_b, vault, token_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !taker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let escrow_data = Escrow::try_from_slice(&escrow.try_borrow_mut_data()?)?;
    let escrow_pda = Pubkey::find_program_address(&[b"escrow", maker.key.as_ref(), escrow_data.seed.to_le_bytes().as_ref()], &crate::ID);

    if escrow.key.ne(&escrow_pda.0) {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_data.maker.ne(&maker.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    if escrow_data.mint_a.ne(&mint_a.key) && escrow_data.mint_b.ne(&mint_b.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let maker_ata_data = spl_token::state::Account::unpack(&maker_ata.try_borrow_mut_data()?)?;
    if maker_ata_data.mint.ne(mint_b.key) && maker_ata_data.owner.ne(maker.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let taker_ata_a_data = spl_token::state::Account::unpack(&taker_ata_a.try_borrow_mut_data()?)?;
    if taker_ata_a_data.mint.ne(mint_a.key) && taker_ata_a_data.owner.ne(taker.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let taker_ata_b_data = spl_token::state::Account::unpack(&taker_ata_b.try_borrow_mut_data()?)?;
    if taker_ata_b_data.mint.ne(mint_b.key) && taker_ata_b_data.owner.ne(taker.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let vault_data = spl_token::state::Account::unpack(&vault.try_borrow_mut_data()?)?;
    if vault_data.mint.ne(mint_a.key) && vault_data.owner.ne(escrow.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let decimals_b = spl_token::state::Mint::unpack(&mint_b.try_borrow_mut_data()?)?.decimals;
    let transfer_ix_a = spl_token::instruction::transfer_checked(
        token_program.key,
        taker_ata_b.key,
        mint_b.key,
        maker_ata.key,
        taker.key,
        &[taker.key],
        escrow_data.amount,
        decimals_b,
    )?;
    invoke(
        &transfer_ix_a, 
        &[taker_ata_b.clone(), mint_b.clone(), maker_ata.clone(), taker.clone(), token_program.clone()]
    )?;

    let decimals_a = spl_token::state::Mint::unpack(&mint_a.try_borrow_mut_data()?)?.decimals;
    let transfer_ix_b = spl_token::instruction::transfer_checked(
        token_program.key,
        vault.key,
        mint_a.key,
        taker_ata_a.key,
        escrow.key,
        &[],
        vault_data.amount,
        decimals_a,
    )?;
    invoke_signed(
        &transfer_ix_b, 
        &[vault.clone(), mint_a.clone(), taker_ata_a.clone(), escrow.clone(), token_program.clone()],
        &[&[b"escrow", maker.key.as_ref(), escrow_data.seed.to_le_bytes().as_ref(), &[escrow_pda.1]]]
    )?;

    let escrow_balance = escrow.lamports();
    *escrow.lamports().borrow_mut() = 0;
    *maker.lamports().borrow_mut() += escrow_balance;

    Ok(())
}