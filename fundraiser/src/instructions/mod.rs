use pinocchio::program_error::ProgramError;

pub mod checker;
pub mod contribute;
pub mod initialize;
pub mod refund;

#[derive(Clone, Copy, Debug)]
pub enum FundraiserInstruction {
    Initialize,
    Contribute,
    Checker,
    Refund,
}

impl TryFrom<&u8> for FundraiserInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FundraiserInstruction::Initialize),
            1 => Ok(FundraiserInstruction::Contribute),
            2 => Ok(FundraiserInstruction::Checker),
            3 => Ok(FundraiserInstruction::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
