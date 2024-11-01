use instructions::EscrowInstructions;
use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

mod instructions;

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222");

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstructions::try_from(discriminator)? {
        EscrowInstructions::Make => make::process(accounts, instruction_data),
        EscrowInstructions::Take => take::process(accounts, instruction_data),
        EscrowInstructions::Refund => take::process(accounts, instruction_data),
    }

    Ok(())
}