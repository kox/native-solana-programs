#[cfg(test)]
mod refund_tests {
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
    use std::mem;

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));

    #[test]
    fn should_fail_when_campaign_still_running() {
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
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Data
        let data = [vec![3]].concat();

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
                AccountMeta::new(authority, false),
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
                    get_ta(&mollusk, mint, contributor, u64::MAX, token_program),
                ), // not used
                (contributor_account, get_contributor(&mollusk, 0u64)), // not used
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
    fn should_fail_when_campaign_ended_reached_goal() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Data
        let data = [vec![3]].concat();

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
                AccountMeta::new(authority, false),
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
                    get_ta(&mollusk, mint, contributor, u64::MAX, token_program),
                ), // not used
                (contributor_account, get_contributor(&mollusk, 0u64)), // not used
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, u64::MIN, u64::MIN, bump),
                ), // slot min -> campaign ended and reached goal -> not refund
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
    fn refund() {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        mollusk_token::token::add_program(&mut mollusk);
        mollusk.sysvars.warp_to_slot(2); // We start in slot 2 so we can test expired (0)
        let (token_program, token_program_account) = mollusk_token::token::keyed_account();

        let maker = Pubkey::new_unique();
        let contributor = Pubkey::new_unique();
        let contributor_ta = Pubkey::new_unique();
        let contributor_account = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let (authority, bump) =
            Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Data
        let data = [vec![3]].concat();

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
                AccountMeta::new(authority, false),
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
                    get_ta(&mollusk, mint, contributor, u64::MIN, token_program),
                ), // Start with no tokens
                (contributor_account, get_contributor(&mollusk, 1_000u64)), // we will refund 1_000u64
                (
                    fundraiser,
                    get_fundraiser(&mollusk, maker, mint, 1_000u64, u64::MIN, bump),
                ), // campaign ended but not reached (remaining > 0) -> refund OK
                (
                    vault,
                    get_ta(&mollusk, mint, authority, 2_000u64, token_program),
                ), // the vault will start with 2000
                (
                    authority,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err()); // It should not fail

        /* let contributor_account = result
        .get_account(&contributor_account)
        .expect("Faiiled to get fundrasier account"); */

        // Check the tokens happened
        let updated_contributor_ta_account = result
            .get_account(&contributor_ta)
            .expect("Failed to find contributor token account");

        // Unpack the updated `contributor_ta` account data to read the token balance
        let updated_contributor_ta_data: spl_token::state::Account =
            solana_sdk::program_pack::Pack::unpack(&updated_contributor_ta_account.data())
                .expect("Failed to unpack contributor token account");

        // Check the balance of `contributor_ta` after contribution
        let expected_balance = 1_000; // Assuming the contributor transferred all tokens to the fundraiser
        assert_eq!(updated_contributor_ta_data.amount, expected_balance);

        // Vault should be 0
        let updated_vault_account = result
            .get_account(&vault)
            .expect("Failed to find vault account");
        let updated_vault_data: spl_token::state::Account =
            solana_sdk::program_pack::Pack::unpack(&updated_vault_account.data())
                .expect("Failed to unpack contributor token account");
        let expected_balance = 1_000; // Assuming the contributor added 1000 and there was 2000, there should be 1000 left

        assert_eq!(updated_vault_data.amount, expected_balance);
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

/*
fn get_instruction(
    accounts
    signer: Pubkey,
    contributor: Pubkey,
    contributor_ta: Pubkey,
    contributor_account: Pubkey,
    fundraiser: Pubkey,
    mint: Pubkey,
    vault: Pubkey,
    num_instruction: u8
) -> Instruction {
    let data = [vec![num_instruction]].concat();

    Instruction::new_with_bytes(
        PROGRAM_ID,
        &data,
        vec![
            AccountMeta::new(contributor, true), */
