use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount: u64,
}

impl Escrow {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8;

    #[inline]
    pub fn init(
        escrow: &AccountInfo, 
        seed: u64, 
        maker: Pubkey, 
        mint_a: Pubkey, 
        mint_b: Pubkey, 
        amount: u64
    ) -> ProgramResult {
        let mut binding = escrow.try_borrow_mut_data()?;
        
        let binding = binding.as_mut();
        
        let escrow_account = bytemuck::try_from_bytes_mut::<Escrow>(binding)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        *escrow_account = Escrow {
            seed,
            maker,
            mint_a,
            mint_b,
            amount,
        };

        Ok(())
    }
}