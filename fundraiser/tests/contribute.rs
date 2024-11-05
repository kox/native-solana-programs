#[cfg(test)]
mod contribute_tests {
    use std::mem;

    use fundraiser::Fundraiser;

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
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

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

        let data = [
            vec![1], // Second instruction (contribute)
            10u64.to_le_bytes().to_vec()
        ].concat();

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

    #[test]
    fn should_fail_when_expired() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

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

        // we are forcing it to be expired to detect error 
        let slot = 0u64;

        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                100_000_000u64.to_le_bytes().to_vec(),
                slot.to_le_bytes().to_vec(),
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        let data = [
            vec![1], // Second instruction (contribute)
            10u64.to_le_bytes().to_vec()
        ].concat();

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

    #[test]
    fn contribute() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

        // Fill out our account data
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

        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );

        // 200 slots until fundraising will end
        let slot = mollusk.sysvars.clock.slot + 200;

        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                100_000_000u64.to_le_bytes().to_vec(),
                slot.to_le_bytes().to_vec(),
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        let data = [
            vec![1], // Second instruction (contribute)
            1_000_000u64.to_le_bytes().to_vec()
        ].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(fundraiser, true),  // we need to modify it
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

        assert!(!result.program_result.is_err());

        // Fixinig the random error as the order is not quarantee
        let fundraiser_result_account = result.get_account(&fundraiser).expect("Failed to find funraiser account");

        // Fundraiser should be own by the program id to be able to modify it
        assert_eq!(*fundraiser_result_account.owner(), program_id);

        // Fundraiser should have a length of 81
        assert_eq!(fundraiser_result_account.data().len(), Fundraiser::LEN);

        // Let's verify the data
        let data = fundraiser_result_account.data();

        // Maker Pubkey
        let pubkey_bytes: [u8; 32] = data[0..32].try_into().expect("Expected 32 bytes for pubkey");
        let maker_pubkey = Pubkey::from(pubkey_bytes);
        assert_eq!(maker_pubkey.to_string(), maker.to_string());

        // Mint Pubkey
        let mint_bytes: [u8; 32]  = data[32..64].try_into().expect("Expecting 8 bytes for mint");
        let mint_pubkey = Pubkey::from(mint_bytes);
        assert_eq!(mint_pubkey.to_string(), mint.to_string());

        // Remaining Amount
        let remaining_amount_bytes: [u8; 8]  = data[64..72].try_into().expect("Expecting 8 bytes for remaining_amount");
        let remaining_amount_result = u64::from_le_bytes(remaining_amount_bytes);
        assert_eq!(remaining_amount_result, 99_000_000u64);

        // Slot
        let slot_bytes: [u8; 8]  = data[72..80].try_into().expect("Expecting 8 bytes for slot");
        let slot_result = u64::from_le_bytes(slot_bytes);
        assert_eq!(slot_result, slot);
    }
}