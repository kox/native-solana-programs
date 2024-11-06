use pinocchio::account_info::AccountInfo;
use solana_sdk::pubkey::Pubkey;


struct Config(*const u8);
    /* mint_x
    mint_y
    mint_lp
    vault_x
    vault_y
    fee: u16 */


impl Config {
    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    #[inline(always)]
    pub fn mint_x(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    #[inline(always)]
    pub fn mint_y(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn mint_lp(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn vault_x(&self) -> Pubkey {
        unsafe { *(self.0.add(96) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn vault_y(&self) -> Pubkey {
        unsafe { *(self.0.add(128) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn fee() -> u16 {
        unsafe { *(self.0.add(160) as *const u16) }

    }

    #[inline(always)]
    pub fn bump(&self) -> Pubkey {
        unsafe { *(self.0.add(162) as *const u8) }
    }
}