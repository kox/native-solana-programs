use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # Publish State
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
pub struct Publish(*const u8);

impl Publish {
    pub const LEN: usize = 32 + 32 + 8 + 1;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn publisher(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    pub fn mint(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn price(&self) -> u64 {
        unsafe { *(self.0.add(64) as *const u64) }
    }

    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(72) as *const u8) }
    }
}
