use pinocchio::program_error::ProgramError;

pub mod initialize;
pub mod swap;
pub mod checker;
pub mod refund;

#[derive(Clone, Copy, Debug)]
pub enum FundraiserInstruction {
    Swap,
    Deposit
    Initialize,
}

impl TryFrom<&u8> for FundraiserInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FundraiserInstruction::Initialize),
            1 => Ok(FundraiserInstruction::Contribute),
            2 => Ok(FundraiserInstruction::Checker),
            3 => Ok(FundraiserInstruction::Refund),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
    }