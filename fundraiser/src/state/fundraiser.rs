use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};


/// # State
///
/// -- Data --
/// > Maker: Pubkey
/// > Mint: Pubkey
/// > RemainingAmount: u64
/// > Slot: u64
/// > bump: u8
///
/// -- Data Logic --
/// [...]
///
pub struct Fundraiser(*const u8);

impl Fundraiser {
    pub const LEN: usize = 81;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn maker(&self) -> Pubkey {
        unsafe  { *(self.0 as *const Pubkey) }
    }

    pub fn mint(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn remaining_amount(&self) -> u64 {
        unsafe { *(self.0.add(64) as *const u64) }
    }

    pub fn slot(&self) -> u64 {
        unsafe { *(self.0.add(72) as *const u64) }
    }

    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(80) as *const u8) }
    }



    /* pub fn remaining_account(&self) -> u64 {
        unsafe  { *(self.0.add(40) as *const Pubkey) }
    } */
}