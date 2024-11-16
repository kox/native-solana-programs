#[path = "./shared.rs"]
mod shared;

#[cfg(test)]
mod deposit_tests {
    use crate::shared;

    use mollusk_svm::{result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    use amm::Config;

    #[test]
    fn deposit() {
        let (mollusk, program_id) = shared::setup();
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let user = Pubkey::new_unique();
        let config = Pubkey::new_unique();
        let (authority, bump) = Pubkey::find_program_address(&[config.as_ref()], &program_id);
        let mint_lp = Pubkey::new_unique();
        let user_x = Pubkey::new_unique();
        let user_y = Pubkey::new_unique();
        let user_lp = Pubkey::new_unique();
        let vault_x = Pubkey::new_unique();
        let vault_y = Pubkey::new_unique();

        let data = [
            vec![1],
            1_000_000_000u64.to_le_bytes().to_vec(),
            1_000_000_000u64.to_le_bytes().to_vec(),
            1_000_000_000u64.to_le_bytes().to_vec(),
            i64::MIN.to_le_bytes().to_vec(),
        ]
        .concat();

        let mut mint_lp_account =
            shared::create_mint_account(&mollusk, authority, 0, 6, true, token_program);

        
        let mut user_x_account = 
            shared::create_token_account(&mollusk, Pubkey::default(), user, 1_000_000_000, token_program);
        
        let mut user_y_account = 
            shared::create_token_account(&mollusk, Pubkey::default(), user, 1_000_000_000, token_program);



    }
}
