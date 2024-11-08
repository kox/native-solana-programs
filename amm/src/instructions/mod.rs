use pinocchio::program_error::ProgramError;

pub mod initialize;
pub mod swap;
pub mod deposit;
pub mod withdraw;
/* pub mod checker;
pub mod refund; */

#[derive(Clone, Copy, Debug)]
pub enum AmmInstruction {
    Swap,
    Deposit,
    Withdraw,
    Initialize,
}

impl TryFrom<&u8> for AmmInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AmmInstruction::Initialize),
            1 => Ok(AmmInstruction::Swap),
            2 => Ok(AmmInstruction::Deposit),
            3 => Ok(AmmInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
    }