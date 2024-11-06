#[cfg(test)]
mod refund_tests {
    use std::mem;

    use fundraiser::{ Fundraiser, Contributor };

    use mollusk_svm::{ program, Mollusk };
    
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount}, 
        account_info::AccountInfo, 
        clock::Slot, 
        deserialize_utils, 
        instruction::{AccountMeta, Instruction}, 
        program_option::COption, pubkey::Pubkey,
        program_pack::Pack,
    };
    
    use spl_token::state::AccountState;

    #[test]
    fn should_fail_when_lower_than_minimun() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        let mut contributor_ta_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint,
                owner: contributor,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            contributor_ta_account.data_as_mut_slice(),
        )
        .unwrap();

        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );

        // Data
        let data = [
            vec![3],
            vec![bump],
        ]
        .concat();

        // We create the vault too
        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        )
        .unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, true),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    contributor,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (contributor_ta, contributor_ta_account),
                (fundraiser, fundraiser_account),
                (mint, mint_account),
                (vault, vault_account),
                (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
                (token_program, token_program_account),
                
            ],
        );

        assert!(result.program_result.is_err()); // It should fail

        // Fill out our account data
        /* let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        ); */
    }

}