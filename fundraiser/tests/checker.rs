#[cfg(test)]
mod checker_tests {
    use mollusk_svm::Mollusk;
    use std::{mem, u64};

    use fundraiser::{ Fundraiser, Contributor };

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
    fn should_fail_when_still_running() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Data
        let data = [vec![2]].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    maker_ta,
                    get_ta(&mollusk, mint, maker, u64::MIN, token_program),
                ), // not used
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, 100_000_000u64, u64::MAX, bump),
                ), // slot max -> campaign still running
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ), // not used                                                           // not used
                (
                    authority,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(result.program_result.is_err()); // It should fail
    }
 
    #[test]
    fn should_fail_when_not_reach_goal() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Data
        let data = [vec![2]].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    maker_ta,
                    get_ta(&mollusk, mint, maker, u64::MIN, token_program),
                ), // not used
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, 100_000u64, u64::MIN, bump),
                ), // slot min -> ended and remaining > 0 -> not success goal
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ), // not used  
                (
                    authority,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(result.program_result.is_err());
    }

    #[test]
    fn should_fail_when_not_maker_tries_to_claim() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let scammer = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        let fundraiser_account = get_fundraiser(&mollusk, maker, mint, u64::MIN, u64::MIN, bump);

        let data = [vec![2]].concat();

        // Let's try to hack it with different signer
        let maker = scammer;

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker, // This is the scammer pubkey ;)
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    maker_ta,
                    get_ta(&mollusk, mint, maker, u64::MIN, token_program),
                ), // not used
                (
                    fundraiser,
                    fundraiser_account,
                ), // slot min -> ended and remaining == 0 -> success goal
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ), // not used  
                (
                    authority,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(result.program_result.is_err());
    }

    #[test]
    fn checker() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let maker_ta = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        let fundraiser_account = get_fundraiser(&mollusk, maker, mint, u64::MIN, u64::MIN, bump);

        let data = [vec![2]].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(maker_ta, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(authority, false),
                AccountMeta::new(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker, // This is the scammer pubkey ;)
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    maker_ta,
                    get_ta(&mollusk, mint, maker, u64::MIN, token_program),
                ), // not used
                (
                    fundraiser,
                    fundraiser_account,
                ), // slot min -> ended and remaining == 0 -> success goal
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ), // not used  
                (
                    authority,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        // Let's verify the vault maker will receive the tokens
        // The vault will be emptied and closed
        // Check the tokens happened
        let updated_maker_ta_account = result
            .get_account(&maker_ta)
            .expect("Failed to find maker token account");

        // Unpack the updated `contributor_ta` account data to read the token balance
        let updated_maker_ta_data: spl_token::state::Account =
            solana_sdk::program_pack::Pack::unpack(&updated_maker_ta_account.data())
                .expect("Failed to unpack maker token account");

        // Check the balance of `maker_ta` after maker
        let expected_balance = 2_000; // Assuming the contributor transferred all tokens to the fundraiser
        assert_eq!(updated_maker_ta_data.amount, expected_balance);

        // Vault should be 0 and no data or lamports
        let updated_vault_account = result
            .get_account(&vault)
            .expect("Failed to find vault account");

        assert_eq!(updated_vault_account.lamports(), 0u64);
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
}
