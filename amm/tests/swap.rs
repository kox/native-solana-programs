#[path = "./shared.rs"]
mod shared;

#[cfg(test)]
mod swap_tests {
    use crate::shared::{self};

    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn swap() {
        let (mollusk, program_id) = shared::setup();
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let user = Pubkey::new_unique();
        let config = Pubkey::new_unique();
        let (authority, bump) = Pubkey::find_program_address(&[config.as_ref()], &program_id);
        let mint_x = Pubkey::new_unique();
        let mint_y = Pubkey::new_unique();
        let user_x = Pubkey::new_unique();
        let user_y = Pubkey::new_unique();
        let vault_from = Pubkey::new_unique();
        let vault_to = Pubkey::new_unique();

        let swap_amount = 1_000_000u64;

        let data = [
            vec![3],                             // amount
            1_000_000u64.to_le_bytes().to_vec(), // mint_x
            1_000u64.to_le_bytes().to_vec(),     // expiration
            i64::MIN.to_le_bytes().to_vec(),
        ]
        .concat();

        let user_x_account =
            shared::create_token_account(&mollusk, mint_x, user, 1_000_000_000, token_program);

        let user_y_account = shared::create_token_account(&mollusk, mint_y, user, 0, token_program);

        let vault_from_account =
            shared::create_token_account(&mollusk, mint_x, authority, 1_000_000_000, token_program);

        let vault_to_account =
            shared::create_token_account(&mollusk, mint_y, authority, 1_000_000_000, token_program);

        let config_account = shared::create_config(
            &mollusk,
            0,
            authority,
            mint_x,
            mint_y,
            Pubkey::new_unique(),
            vault_from,
            vault_to,
            1_000u16,
            bump,
            program_id,
        );

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(user, true),
                AccountMeta::new(authority, false),
                /* AccountMeta::new(mint_lp, false), */
                AccountMeta::new(user_x, false),
                AccountMeta::new(user_y, false),
                /* AccountMeta::new(user_lp, false), */
                AccountMeta::new(vault_from, false),
                AccountMeta::new(vault_to, false),
                AccountMeta::new(config, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    user,
                    AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default()),
                ),
                (
                    authority,
                    AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default()),
                ),
                (user_x, user_x_account),
                (user_y, user_y_account),
                (vault_from, vault_from_account),
                (vault_to, vault_to_account),
                (config, config_account),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        shared::expect_token_balance(&result, user_x, 999000000u64);
        shared::expect_token_balance(&result, user_y, 899100u64);
        shared::expect_token_balance(&result, vault_from, 1001000000u64);
        shared::expect_token_balance(&result, vault_to, 999100900u64);
    }
}
