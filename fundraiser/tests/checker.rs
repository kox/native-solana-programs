#[cfg(test)]
mod checker_tests {
    use std::mem;
    use mollusk_svm::Mollusk;
    use pinocchio::instruction::{Seed, Signer};
    use pinocchio_token::{instructions::Transfer, state::{Token, TokenAccount}};
    use solana_sdk::{
        account::{AccountSharedData, WritableAccount}, 
        instruction::{AccountMeta, Instruction}, 
        program_option::COption, pubkey::Pubkey,
        program_pack::Pack,
    };
    
    use spl_token::state::AccountState;
    use fundraiser::Fundraiser;

    #[test]
    fn should_fail_when_still_running() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

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
        
        // Data
        let data = [
            vec![2],
            vec![bump],
        ]
        .concat();
 
        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (fundraiser, fundraiser_account),
            ],
        );

        assert!(result.program_result.is_err());

    }

    #[test]
    fn should_fail_when_not_reach_goal() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk_token::token::add_program(&mut mollusk);

        mollusk.warp_to_slot(2);

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

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
                1_000u64.to_le_bytes().to_vec(), // not remaining tokens left for the goal
                0u64.to_le_bytes().to_vec(), // expired
                bump.to_le_bytes().to_vec(),
            ]
            .concat(),
        );
        
        // Data
        let data = [
            vec![2],
            vec![bump],
        ]
        .concat();
 
        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (fundraiser, fundraiser_account),
            ],
        );

        assert!(result.program_result.is_err());

    }

    #[test]
    fn should_fail_when_not_maker_tries_to_claim() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk.warp_to_slot(2);
        
        mollusk_token::token::add_program(&mut mollusk);

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let scammer = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

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
                0u64.to_le_bytes().to_vec(), // not remaining tokens left for the goal
                u64::MIN.to_le_bytes().to_vec(), // expired
                bump.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        

        // Check that the mint of the vault is the mint to raise
        // assert_eq!(&vault_account.mint(), &fundraiser_account.mint_to_raise());

        
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

        let mut maker_ta_account = AccountSharedData::new(
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
                owner: maker,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ta_account.data_as_mut_slice(),
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
 
        // Data
        let data = [
            vec![2],
            vec![bump],
        ]
        .concat();
 
        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(scammer, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(token_program, false), 
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    scammer,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (maker_ta, maker_ta_account),
                (fundraiser, fundraiser_account), 
                (vault, vault_account),
                (token_program, token_program_account), 
            ],
        );

        assert!(result.program_result.is_err());

    }

    #[test]
    fn checker() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        mollusk.warp_to_slot(2);
        
        mollusk_token::token::add_program(&mut mollusk);

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let scammer = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

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
                0u64.to_le_bytes().to_vec(), // not remaining tokens left for the goal
                u64::MIN.to_le_bytes().to_vec(), // expired
                bump.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        

        // Check that the mint of the vault is the mint to raise
        // assert_eq!(&vault_account.mint(), &fundraiser_account.mint_to_raise());

        
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

        let mut maker_ta_account = AccountSharedData::new(
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
                owner: maker,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ta_account.data_as_mut_slice(),
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
 
        // Data
        let data = [
            vec![2],
            vec![bump],
        ]
        .concat();
 
        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(scammer, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(token_program, false), 
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    scammer,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (maker_ta, maker_ta_account),
                (fundraiser, fundraiser_account), 
                (vault, vault_account),
                (token_program, token_program_account), 
            ],
        );

        assert!(!result.program_result.is_err());

    }
}