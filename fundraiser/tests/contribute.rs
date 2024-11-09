#[cfg(test)]
mod contribute_tests {
    use std::{mem, u64};

    use fundraiser::{Contributor, Fundraiser};

    use mollusk_svm::Mollusk;

    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };

    use spl_token::state::AccountState;

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));

    #[test]
    fn should_fail_when_lower_than_minimun() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &PROGRAM_ID).unwrap();

        let data = [
            vec![1], // Second instruction (contribute)
            1u64.to_le_bytes().to_vec(),  // we contribute with only 1 token fail
        ]
        .concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(fundraiser, false),
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
                (
                    contributor_ta,
                    get_ta(&mollusk, mint, contributor, 1_000_000u64, token_program),
                ),
                (contributor_account, get_contributor(&mollusk, u64::MIN)), // first time contributing
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, 1_000_000_000u64, u64::MAX, bump),
                ),
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ),
                (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(result.program_result.is_err()); // It should fail
    }

    #[test]
    fn should_fail_when_expired() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &PROGRAM_ID).unwrap();

        let data = [
            vec![1], // Second instruction (contribute)
            1_000_000u64.to_le_bytes().to_vec(),  // we contribute with only 1 token fail
        ]
        .concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(fundraiser, false),
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
                (
                    contributor_ta,
                    get_ta(&mollusk, mint, contributor, 1_000_000u64, token_program),
                ),
                (contributor_account, get_contributor(&mollusk, u64::MIN)), // first time contributing
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, 1_000_000_000u64, u64::MIN, bump),
                ),
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ),
                (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(result.program_result.is_err()); // It should fail
    }

    #[test]
    fn contribute() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &PROGRAM_ID).unwrap();

        let data = [
            vec![1], // Second instruction (contribute)
            1_000_000u64.to_le_bytes().to_vec()
        ].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(contributor_ta, false),
                AccountMeta::new(contributor_account, false),
                AccountMeta::new(fundraiser, false),  // we need to modify it
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
                (contributor_ta, get_ta(&mollusk, mint, contributor, 1_000_000u64, token_program)),
                (contributor_account, get_contributor(&mollusk, u64::MIN)), // first time contributing
                (fundraiser, get_fundraiser(&mollusk, maker, mint, 1_000_000_000u64, u64::MAX, bump)),
                (vault, get_ta(&mollusk, mint, authority, 2_000u64, token_program)),
                (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        // Fixinig the random error as the order is not quarantee
        let fundraiser_result_account = result.get_account(&fundraiser).expect("Failed to find funraiser account");

        // Fundraiser should be own by the program id to be able to modify it
        assert_eq!(*fundraiser_result_account.owner(), PROGRAM_ID);

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
        assert_eq!(remaining_amount_result, 999_000_000u64);


        // Check the tokens happened
        let updated_contributor_ta_account = result
            .get_account(&contributor_ta)
            .expect("Failed to find contributor token account");
        
        // Unpack the updated `contributor_ta` account data to read the token balance
        let updated_contributor_ta_data: spl_token::state::Account = solana_sdk::program_pack::Pack::unpack(
            &updated_contributor_ta_account.data(),
        ).expect("Failed to unpack contributor token account");

        // Check the balance of `contributor_ta` after contribution
        let expected_balance = 0; // Assuming the contributor transferred all tokens to the fundraiser
        assert_eq!(updated_contributor_ta_data.amount, expected_balance);

        // Fixinig the random error as the order is not quarantee
        let contributor_result_account = result.get_account(&contributor_account).expect("Failed to find contributor account");

        // contributor account should be own by the program id to be able to modify it
        assert_eq!(*contributor_result_account.owner(), PROGRAM_ID);

        // contributor account should have a length of 8
        assert_eq!(contributor_result_account.data().len(), Contributor::LEN);

        // Let's verify the data
        let contributor_account_data = contributor_result_account.data();

        // verify we updated the contribution
        let contribution_amount_bytes: [u8; 8]  = contributor_account_data[0..8].try_into().expect("Expecting 8 bytes for amount");
        let contribution_amount_result = u64::from_le_bytes(contribution_amount_bytes);
        assert_eq!(contribution_amount_result, 1_000_000u64);
    }

    fn get_fundraiser(
        mollusk: &Mollusk,
        maker: Pubkey,
        mint: Pubkey,
        goal: u64,
        end_slot: u64,
        bump: u8,
    ) -> AccountSharedData {
        let mut fundraiser_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(mem::size_of::<Fundraiser>()),
            mem::size_of::<Fundraiser>(),
            &self::PROGRAM_ID,
        );
        fundraiser_account.set_data_from_slice(
            &[
                maker.to_bytes().to_vec(),
                mint.to_bytes().to_vec(),
                goal.to_le_bytes().to_vec(),
                end_slot.to_le_bytes().to_vec(), // Maximum slot so for sure it should fail
                bump.to_le_bytes().to_vec(),
            ]
            .concat(),
        );

        fundraiser_account
    }

    fn get_ta(
        mollusk: &Mollusk,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        token_program: Pubkey,
    ) -> AccountSharedData {
        let mut ta_account = AccountSharedData::new(
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
                owner,
                amount,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            ta_account.data_as_mut_slice(),
        )
        .unwrap();

        ta_account
    }

    fn get_contributor(mollusk: &Mollusk, amount: u64) -> AccountSharedData {
        let mut contributor_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(Contributor::LEN),
            Contributor::LEN,
            &PROGRAM_ID,
        );
        contributor_account.set_data_from_slice(&[amount.to_le_bytes().to_vec()].concat());

        contributor_account
    }
}
