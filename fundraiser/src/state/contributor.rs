use pinocchio::account_info::AccountInfo;


/// # State
///
/// -- Data --
/// > amount: u64
/// > bump: u8
///
/// -- Data Logic --
/// [...]
///
pub struct Contributor(*const u8);

impl Contributor {
    pub const LEN: usize = 9;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn amount(&self) -> u64 {
        unsafe  { *(self.0 as *const u64) }
    }

    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(8) as *const u8) }
    }
}