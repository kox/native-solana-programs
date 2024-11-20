#![feature(asm_experimental_arch)]
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
use instructions::*;

mod state;
pub use state::*;

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MarketplaceInstruction::try_from(discriminator)? {
        MarketplaceInstruction::Initialize => initialize(accounts, data),
        MarketplaceInstruction::Publish => publish(accounts, data),
        MarketplaceInstruction::Unpublish => unpublish(accounts, data),
        MarketplaceInstruction::Purchase => purchase(accounts, data),
    }
}
