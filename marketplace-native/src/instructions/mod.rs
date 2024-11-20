pub mod initialize;
pub use initialize::*;

pub mod publish;
pub use publish::*;

pub mod unpublish;
pub use unpublish::*;

pub mod purchase;
pub use purchase::*;
 
use pinocchio::program_error::ProgramError;

#[derive(Clone, Copy, Debug)]
pub enum MarketplaceInstruction {
    Initialize,
    Publish,
    Unpublish,
    Purchase,
}

impl TryFrom<&u8> for MarketplaceInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MarketplaceInstruction::Initialize),
            1 => Ok(MarketplaceInstruction::Publish),
            2 => Ok(MarketplaceInstruction::Unpublish),
            3 => Ok(MarketplaceInstruction::Purchase),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
