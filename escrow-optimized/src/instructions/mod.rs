use pinocchio::program_error::ProgramError;

pub mod make;
pub mod take;
pub mod refund;

pub enum EscrowInstructions {
    Make,
    Take,
    Refund
}

impl TryFrom<&u8> for EscrowInstructions {
    type Error;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Make),
            1 => Ok(Self::Take),
            2 => Ok(Self::Refund),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
    }