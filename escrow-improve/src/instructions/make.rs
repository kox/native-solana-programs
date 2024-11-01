use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey, sysvars::rent::Rent};
use pinocchio::sysvars::Sysvar;

use crate::{ constants::SEED_OFFSET, Escrow, ID };

pub fn process_make_instruction(
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> Result<(), ProgramError> {
    let [
        maker, 
        escrow, 
        mint_a, 
        mint_b, 
        maker_ata, 
        vault, 
        _token_program, 
        _system_program
    ] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let seed: u64;
    let amount: u64;

    unsafe {
        seed = *(instruction_data.as_ptr() as *const u64);
        amount = *(instruction_data.as_ptr().add(SEED_OFFSET) as *const u64);
    }

    let (_, bump) = pubkey::find_program_address(&[
        b"escrow", 
        maker.key().as_ref(), 
        seed.to_le_bytes().as_ref()
    ], &ID);

    let seed_binding = seed.to_le_bytes();
    let bump_binding = bump.to_le_bytes();
    let signer_seeds = [
        Seed::from(b"escrow"), 
        Seed::from(maker.key().as_ref()), 
        Seed::from(seed_binding.as_ref()),
        Seed::from(bump_binding.as_ref())
    ];

    let signer = [Signer::from(&signer_seeds)];


    let minimum_balance = Rent::get()?.minimum_balance(Escrow::LEN);

    pinocchio_system::instructions::CreateAccount{
        from: maker,
        to: escrow,
        lamports: minimum_balance,
        space: Escrow::LEN as u64,
        owner: &ID,
    }
    .invoke_signed(&signer);

    Escrow::init(
        escrow,
        seed,
        *maker.key(),
        *mint_a.key(),
        *mint_b.key(),
        amount,
    )?;

    pinocchio_token::instructions::Transfer {
        from: maker_ata,
        to: vault,
        authority: maker,
        amount,
    }
    .invoke()?;
    
    Ok(())
}