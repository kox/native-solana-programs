use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

/// # Marketplace State
///
/// -- Data --
/// > Maker: Pubkey
/// > fee: u64
/// > bump: u8
/// > treasury_bump: u8
///
/// -- Data Logic --
/// [...]
///
pub struct Marketplace(*const u8);

impl Marketplace {
    pub const LEN: usize = 32   // maker 
        + 8                     // fee 
        + 1                     // bump
        + 1; // treasury_bump

    #[inline(always)]
    pub fn init(&self, data: &[u8; Self::LEN]) -> ProgramResult {
        unsafe { *(self.0 as *mut [u8; Self::LEN]) = *data };
        Ok(())
    }

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    #[inline(always)]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Ok(Self::from_account_info_unchecked(account_info))
    }

    // We store who owns the marketplace
    #[inline(always)]
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    // How much the marketplace will retain per purchase
    #[inline(always)]
    pub fn fee(&self) -> u64 {
        unsafe { *(self.0.add(32) as *const u64) }
    }

    #[inline(always)]
    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(40) as *const u8) }
    }

    // To store the SOL
    #[inline(always)]
    pub fn treasury_bump(&self) -> u8 {
        unsafe { *(self.0.add(41) as *const u8) }
    }
}
