use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::pubkey::Pubkey;
use pinocchio::{program_error::ProgramError, ProgramResult};

mod instructions;
use instructions::AmmInstruction;
use instructions::{
    deposit::deposit, initialize::initialize, lock::lock, swap::swap, withdraw::withdraw,
};

mod state;
pub use state::*;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub const ID: [u8; 32] =
    five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match AmmInstruction::try_from(discriminator)? {
        AmmInstruction::Initialize =>   initialize(accounts, data),
        AmmInstruction::Deposit =>      deposit(accounts, data),
        AmmInstruction::Withdraw =>     withdraw(accounts, data),
        AmmInstruction::Swap =>         swap(accounts, data),
        AmmInstruction::Lock =>         lock(accounts),
    }
}