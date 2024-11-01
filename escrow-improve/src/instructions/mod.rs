pub mod make;
pub mod refund;
pub mod take;

pub use make::*;
pub use refund::*;
pub use take::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EscrowInstruction {
    MakeInstruction = 0,
    TakeInstruction = 1,
    RefundInstruction = 2,
}

impl From<u8> for EscrowInstruction {
    fn from(instruction: u8) -> Self {
        match instruction {
            0 => Self::MakeInstruction,
            1 => Self::TakeInstruction,
            2 => Self::RefundInstruction,
            _ => panic!("Wrong Instruction")
        }
    }
}