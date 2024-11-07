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
    fn should_fail_when_still_running() {
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

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );

        // We create a fundraiser account to store the data
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                100_000_000u64.to_le_bytes().to_vec(),
                u64::MAX.to_le_bytes().to_vec(), // Maximum slot so for sure it should fail
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // We create a mint token account and define the token data
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

        // We create a contributor token account and add some tokens
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

        // We create a fundraiser account to store the data
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                100_000_000u64.to_le_bytes().to_vec(),
                u64::MAX.to_le_bytes().to_vec(), // Maximum slot so for sure it should fail
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: vault,
                amount: 1_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

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
                owner: vault,
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
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
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
                (contributor_account, 
                    AccountSharedData::new(
                        mollusk
                            .sysvars
                            .rent
                            .minimum_balance(Contributor::LEN),
                        Contributor::LEN,
                        &program_id,
                    ),
                ),
                (fundraiser, fundraiser_account),
                (vault, vault_account),
                (token_program, token_program_account),
                
            ],
        );

        assert!(result.program_result.is_err()); // It should fail
    }

    #[test]
    fn should_fail_when_campaign_ended_without_reaching_goal() {
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

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );

        // We create a fundraiser account to store the data
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                100_000_000u64.to_le_bytes().to_vec(),
                0u64.to_le_bytes().to_vec(), // we encorage that it has ended
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // We create a mint token account and define the token data
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

        // We create a contributor token account and add some tokens
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

        // We create a fundraiser account to store the data
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                1u64.to_le_bytes().to_vec(), // still has remaining amount
                u64::MAX.to_le_bytes().to_vec(), // Maximum slot so for sure it should fail
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: vault,
                amount: 1_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

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
                owner: vault,
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
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
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
                (contributor_account, 
                    AccountSharedData::new(
                        mollusk
                            .sysvars
                            .rent
                            .minimum_balance(Contributor::LEN),
                        Contributor::LEN,
                        &program_id,
                    ),
                ),
                (fundraiser, fundraiser_account),
                (vault, vault_account),
                (token_program, token_program_account),
                
            ],
        );

        assert!(result.program_result.is_err()); // It should fail
    }

    #[test]
    fn refund() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);

        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

        // Vault starts with 2000
        let mut vault_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: vault,
                amount: 2_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        ).unwrap();

        // fundraiser has ended and it was not successful so the contributor can get refunded
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &program_id,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                1u64.to_le_bytes().to_vec(), // still has remaining amount
                u64::MIN.to_le_bytes().to_vec(), // Maximum slot so for sure it should fail
                1u8.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // We create a mint token account and define the token data
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

        // We create a contributor token account with 0 tokens contribution
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
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            contributor_ta_account.data_as_mut_slice(),
        )
        .unwrap();  

        let mut contributor_account_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(Contributor::LEN),
            Contributor::LEN,
            &program_id,
        );
        contributor_account_account.set_data_from_slice(
            &[
                1_000u64.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        // Data
        let data = [
            vec![3],
            vec![bump],
        ]
        .concat();
 
        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(contributor_account, true),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
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
                (contributor_account, contributor_account_account),
                (fundraiser, fundraiser_account),
                (vault, vault_account),
                (token_program, token_program_account),
                
            ],
        );

        assert!(!result.program_result.is_err()); // It should not fail

        let contributor_account = result
            .get_account(&contributor_account)
            .expect("Faiiled to get fundrasier account");



       /*  let fundraiser_account = result
            .get_account(&fundraiser)
            .expect("Faiiled to get fundrasier account");

        let fundraiser_account_data = Fundraiser::from_account_shared_data_unchecked(fundraiser_account);
        println!("MAKER: {}", fundraiser_account_data.maker()) */

        // Check the tokens happened
        let updated_contributor_ta_account = result
            .get_account(&contributor_ta)
            .expect("Failed to find contributor token account");
        
        // Unpack the updated `contributor_ta` account data to read the token balance
        let updated_contributor_ta_data: spl_token::state::Account = solana_sdk::program_pack::Pack::unpack(
            &updated_contributor_ta_account.data(),
        ).expect("Failed to unpack contributor token account");

        // Check the balance of `contributor_ta` after contribution
        let expected_balance = 1_000; // Assuming the contributor transferred all tokens to the fundraiser
        assert_eq!(updated_contributor_ta_data.amount, expected_balance);

        // Vault should be 0
        let updated_vault_account = result
            .get_account(&vault)
            .expect("Failed to find vault account");
        let updated_vault_data: spl_token::state::Account = solana_sdk::program_pack::Pack::unpack(
            &updated_vault_account.data(),
        ).expect("Failed to unpack contributor token account");
        let expected_balance = 1_000; // Assuming the contributor added 1000 and there was 2000, there should be 1000 left
        
        assert_eq!(updated_vault_data.amount, expected_balance);

        // How can i test that contributor account was closed?
        /* let updated_contributor_account = result
            .get_account(contributor_account)
            .expect("Failed to read contributor account"); */

    }

}