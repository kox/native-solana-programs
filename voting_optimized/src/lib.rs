use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
use instructions::*;

/* use upvote::upvote;
use downvote::downvote;
 */
mod state;

pub use state::VoteState;

const ID: Pubkey = five8_const::decode_32_const("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, _data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VoteInstruction::try_from(discriminator)? {
        VoteInstruction::UpVote => upvote::upvote(accounts),
        VoteInstruction::DownVote => downvote::downvote(accounts),
    }
}
