use pinocchio::program_error::ProgramError;

pub mod downvote;
pub mod upvote;

#[derive(Clone, Copy, Debug)]
pub enum VoteInstruction {
    UpVote,
    DownVote,
    // Initialize,  Do you really need i? I don't think so. We can create it offchain and avoid CPI System Program
}

impl TryFrom<&u8> for VoteInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VoteInstruction::UpVote),
            1 => Ok(VoteInstruction::DownVote),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
