#[path = "./shared.rs"]
mod shared;

#[cfg(test)]
mod deposit_tests {
    use crate::shared::{self, create_token_account};

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
        let mint_x = Pubkey::new_unique();
        let mint_y = Pubkey::new_unique();
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
            shared::create_token_account(&mollusk, mint_x, user, 1_000_000_000, token_program);
        
        let mut user_y_account = 
            shared::create_token_account(&mollusk, mint_y, user, 1_000_000_000, token_program);

        let mut vault_x_account = 
            shared::create_token_account(&mollusk, mint_x, authority, 0, token_program);

        let mut vault_y_account = 
            shared::create_token_account(&mollusk, mint_y, authority, 0, token_program);
        
        let mut user_lp_account = 
            shared::create_token_account(&mollusk, mint_lp, user, 0, token_program);
        
        let config_account = shared::create_config(&mollusk, 0, authority, mint_x, mint_y, mint_lp, vault_x, vault_y, 1_000u16, bump, program_id);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(user, true),
                AccountMeta::new(authority, false),
                AccountMeta::new(mint_lp, false),
                AccountMeta::new(user_x, false),
                AccountMeta::new(user_y, false),
                AccountMeta::new(user_lp, false),
                AccountMeta::new(vault_x, false),
                AccountMeta::new(vault_y, false),
                AccountMeta::new(config, false),
                AccountMeta::new(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (user, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
                (authority, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
                (mint_lp, mint_lp_account),
                (user_x, user_x_account),
                (user_y, user_y_account),
                (user_lp, user_lp_account),
                (vault_x, vault_x_account),
                (vault_y, vault_y_account),
                (config, config_account),
                (token_program, token_program_account),
            ],
            &[Check::success()]
        );
    
    }
}
