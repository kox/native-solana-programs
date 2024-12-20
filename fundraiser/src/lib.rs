#![feature(asm_experimental_arch)]
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
use instructions::*;

use checker::checker;
use contribute::contribute;
use initialize::initialize;
use refund::refund;

mod constants;
mod state;

pub use constants::*;
pub use state::Contributor;
pub use state::Fundraiser;

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222222");
/* const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";
 */
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(discriminator)? {
        FundraiserInstruction::Initialize => initialize(accounts, data),
        FundraiserInstruction::Contribute => contribute(accounts, data),
        FundraiserInstruction::Checker => checker(accounts, data),
        FundraiserInstruction::Refund => refund(accounts, data),
    }
}
