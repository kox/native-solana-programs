use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod initialize;
pub mod lock;
pub mod swap;
pub mod withdraw;

#[derive(Clone, Copy, Debug)]
pub enum AmmInstruction {
    Initialize,
    Deposit,
    Withdraw,
    Swap,
    Lock,
}

impl TryFrom<&u8> for AmmInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Deposit),
            2 => Ok(Self::Withdraw),
            3 => Ok(Self::Swap),
            4 => Ok(Self::Lock),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
