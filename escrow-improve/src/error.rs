use pinocchio::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum EscrowError {
    #[error("Escrow account mismatch")]
    EscrowAccountMismatch
}

impl From<EscrowError> for ProgramError {
    fn from(error: EscrowError) -> Self {
        ProgramError::Custom(error as u32)
    }
}