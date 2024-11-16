
pub mod shared;  // Exposes the `setup` module

// Re-export the `setup` function for easy access in other tests
pub use shared::setup;

/* 
#[cfg(test)]
mod initialize;
 */



/* #[cfg(test)]
mod tests {
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
    fn initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mollusk = Mollusk::new(&program_id, "../target/deploy/amm");

        let config = Pubkey::new_unique();

        let data = [
            vec![0],
            vec![0],
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),            
            u16::MAX.to_le_bytes().to_vec(),
            u8::MAX.to_le_bytes().to_vec(),
        ]
        .concat();


        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(config, true),
            ],
        );

        let lamports = mollusk.sysvars.rent.minimum_balance(Config::LEN);

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (config, AccountSharedData::new(lamports, Config::LEN, &program_id)),
            ],
            &[Check::success()]
        );
    }

    #[test]
    fn deposit() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/amm");
        
        mollusk_token::token::add_program(&mut mollusk);
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

        let mint_lp_account = create_mint_account();

        let mut mint_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );

        /* let mut account = create_account(0, spl_token::state::Mint::LEN, &spl_token::id()); */
        spl_token::state::Mint {
            mint_authority: COption::Some(authority),
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(mint_lp_account.data_as_mut_slice());
 */

        /* solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        )
        .unwrap(); */

        /* let mut mint_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::id(),
        ); */
        /* spl_token::state::Mint::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(authority),
                supply: 0,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_lp_account.data_as_mut_slice(),
        ).unwrap(); */
   /*  }

}
 */
/* pub fn pack_mint(mint_authority: &Pubkey, supply: u64) -> AccountSharedData {
    let mut account = create_account(0, spl_token::state::Mint::LEN, &spl_token::id());
    spl_token::state::Mint {
        mint_authority: COption::Some(*mint_authority),
        supply,
        decimals: 9,
        is_initialized: true,
        freeze_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());
    account
} */

    /* #[test]
    fn deposit() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/amm");
        
        mollusk_token::token::add_program(&mut mollusk);
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

        let mut mint_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::id(),
        );
        spl_token::state::Mint::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(authority),
                supply: 0,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_lp_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_x_account = AccountSharedData::new(
            mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_x_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_y_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_y_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_x_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_x_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_y_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_y_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: mint_lp,
                owner: user,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_lp_account.data_as_mut_slice(),
        ).unwrap();

        let mut config_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Config::LEN),
            Config::LEN,
            &program_id,
        );

        let mut config_data = [0u8; Config::LEN];
        config_data[0] = 0;
        config_data[1..33].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[97..129].copy_from_slice(&mint_lp.to_bytes());
        config_data[129..161].copy_from_slice(&vault_x.to_bytes());
        config_data[161..193].copy_from_slice(&vault_y.to_bytes());
        config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
        config_data[195] = bump; 

        config_account.set_data_from_slice(&config_data);

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
    } */

/*
    #[test]
    #[ignore = "working"]
    fn withdraw() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/amm");
        
        mollusk_token::token::add_program(&mut mollusk);
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
            vec![2],
            1_000_000u64.to_le_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
            i64::MIN.to_le_bytes().to_vec(),
        ]
        .concat();

        let mut mint_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::id(),
        );
        spl_token::state::Mint::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(authority),
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_lp_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_x_account = AccountSharedData::new(
            mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_x_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_y_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_y_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_x_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_x_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_y_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_y_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_lp_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: mint_lp,
                owner: user,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_lp_account.data_as_mut_slice(),
        ).unwrap();

        let mut config_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Config::LEN),
            Config::LEN,
            &program_id,
        );

        let mut config_data = [0u8; Config::LEN];
        config_data[0] = 0;
        config_data[1..33].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[97..129].copy_from_slice(&mint_lp.to_bytes());
        config_data[129..161].copy_from_slice(&vault_x.to_bytes());
        config_data[161..193].copy_from_slice(&vault_y.to_bytes());
        config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
        config_data[195] = bump; 

        config_account.set_data_from_slice(&config_data);

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

    #[test]
    #[ignore = "working"]
    fn swap() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/native_amm");
        
        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let user = Pubkey::new_unique();
        let config = Pubkey::new_unique();
        let (authority, bump) = Pubkey::find_program_address(&[config.as_ref()], &program_id);
        let user_x = Pubkey::new_unique();
        let user_y = Pubkey::new_unique();
        let vault_from = Pubkey::new_unique();
        let vault_to = Pubkey::new_unique();

        let data = [
            vec![3],
            1_000_000u64.to_le_bytes().to_vec(),
            1_000u64.to_le_bytes().to_vec(),
            i64::MIN.to_le_bytes().to_vec(),
        ]
        .concat();

        let mut user_x_account = AccountSharedData::new(
            mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_x_account.data_as_mut_slice(),
        ).unwrap();

        let mut user_y_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: user,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            user_y_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_from_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_from_account.data_as_mut_slice(),
        ).unwrap();

        let mut vault_to_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            spl_token::state::Account {
                mint: Pubkey::default(),
                owner: authority,
                amount: 1_000_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_to_account.data_as_mut_slice(),
        ).unwrap();

        let mut config_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Config::LEN),
            Config::LEN,
            &program_id,
        );

        let mut config_data = [0u8; Config::LEN];
        config_data[0] = 0;
        config_data[1..33].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[97..129].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[129..161].copy_from_slice(&vault_from.to_bytes());
        config_data[161..193].copy_from_slice(&vault_to.to_bytes());
        config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
        config_data[195] = bump; 

        config_account.set_data_from_slice(&config_data);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(user, true),
                AccountMeta::new(authority, false),
                AccountMeta::new(user_x, false),
                AccountMeta::new(user_y, false),
                AccountMeta::new(vault_from, false),
                AccountMeta::new(vault_to, false),
                AccountMeta::new(config, false),
                AccountMeta::new(token_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (user, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
                (authority, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
                (user_x, user_x_account),
                (user_y, user_y_account),
                (vault_from, vault_from_account),
                (vault_to, vault_to_account),
                (config, config_account),
                (token_program, token_program_account),
            ],
            &[Check::success()]
        );
    }

    #[test]
    #[ignore = "working"]
    fn lock() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mollusk = Mollusk::new(&program_id, "../target/deploy/amm");

        let authority = Pubkey::new_unique();
        let config = Pubkey::new_unique();

        let data = [4];

        let mut config_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Config::LEN),
            Config::LEN,
            &program_id,
        );

        let mut config_data = [0u8; Config::LEN];
        config_data[0] = 0;
        config_data[1..33].copy_from_slice(&authority.to_bytes());
        config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[97..129].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[129..161].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[161..193].copy_from_slice(&Pubkey::default().to_bytes());
        config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
        config_data[195] = u8::MAX; 

        config_account.set_data_from_slice(&config_data);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(authority, true),
                AccountMeta::new(config, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (authority, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
                (config, config_account),
            ],
            &[Check::success()]
        );
    }

}
*/