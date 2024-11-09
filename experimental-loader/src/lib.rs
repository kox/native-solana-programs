use five8_const::decode_32_const;
use pinocchio::{account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey, ProgramResult};

const ID: Pubkey = decode_32_const("22222222222222222222222222222222222222222222");


macro_rules! define_account {
    (
        $name:ident {
            $($field:ident: ($ty:ty, $offset:expr)),* $(,)?
        }
    ) => {
        pub struct $name(*const u8);

        impl $name {
            pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
                unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
            }

            pub fn from_account_info(account_info: &AccountInfo) -> Self {
                assert_eq!(account_info.data_len(), Self::LEN);
                assert_eq!(account_info.owner(), &ID);
                Self::from_account_info_unchecked(account_info)
            }

            pub const LEN: usize = {
                0 $(+ std::mem::size_of::<$ty>() + $offset)*
            };

            $(
                pub fn $field(&self) -> &$ty {
                    unsafe { &*(self.0.add($offset) as *const $ty) }
                }

                pub fn set_$field(&self, value: $ty) {
                    unsafe {
                        let ptr = self.0.add($offset) as *mut $ty;
                        *ptr = value;
                    }
                }
            )*
        }
    };
}

// Define the Fundraiser account layout
define_account! {
    MyAccount {
        maker: (Pubkey, 0),          // Pubkey starting at offset 0
        amount: (u64, 32),           // u64 at offset 32
        bump: (u8, 40),              // u8 at offset 40
    }
}


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    let account_info_iter = &mut accounts.iter();
    let account_info = next_account_info(account_info_iter)?;

    // Load the account as Fundraiser
    let my_account = MyAccount::from_account_info(account_info);

    // Display the maker's Pubkey as a message
    msg!("Maker Pubkey: {:?}", my_account.maker());

    // Optionally display other values
    // msg!("Amount: {}", my_account.amount());
    // msg!("Bump: {}", my_account.0.
    
    Ok(())
}
/* 
#[cfg(test)]
mod tests {
    use super::*; // Import the Fundraiser definition and process_instruction
    use pinocchio::{
        account_info::AccountInfo, pubkey::Pubkey, msg,
        rent::Rent, sysvar::rent::ID as RENT_ID,
    };
    use solana_sdk::{account::Account, account_info::IntoAccountInfo};

    #[test]
    fn test_fundraiser_account() {
        // Mock Pubkeys
        let maker_pubkey = Pubkey::new_unique();
        let owner_pubkey = ID;

        // Mock account data
        let mut account_data = vec![0u8; MyAccount::LEN];
        account_data[0..32].copy_from_slice(maker_pubkey.as_ref()); // Set maker Pubkey
        account_data[32..40].copy_from_slice(&100_u64.to_le_bytes()); // Set amount
        account_data[40] = 1; // Set bump

        // Mock Account
        let mut lamports = 1000;
        let account = Account {
            lamports,
            data: account_data,
            owner: owner_pubkey,
            ..Account::default()
        };

        let account_info: AccountInfo = (&Pubkey::new_unique(), &mut account).into_account_info();

        // Load Fundraiser from account info
        let my_account = MyAccount::from_account_info(&account_info);

        // Assertions
        assert_eq!(my_account.maker(), &maker_pubkey);
        assert_eq!(*my_account.amount(), 100_u64);
        assert_eq!(*my_account.bump(), 1_u8);

        msg!("All tests passed for Fundraiser account!");
    }
} */
/* pub struct MyAccount(*const u8);
    pub const LEN: usize = 41; // Adjust as needed based on the struct layout

    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }
 */




