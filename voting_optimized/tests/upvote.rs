#[cfg(test)]
mod upvote_tests {
    use voting_optimized::VoteState;

    use mollusk_svm::{program, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount},
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));

    #[test]
    fn vote_up_first_time() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/voting_optimized");
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let user = Pubkey::new_unique();
        let voting_account = Pubkey::new_unique();

        let data = [
            vec![0], // upvote
        ]
        .concat();

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                // AccountMeta::new(user, true),
                AccountMeta::new(voting_account, false), // It should be a signer because this account shouldn't exist yet
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        // we will create an account rent exempt with enough lamports to cover basic accountinfo plus fundraiser data
        let lamports = mollusk.sysvars.rent.minimum_balance(8);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                /* (
                    user,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ), */
                (voting_account, get_voting_account(&mollusk, u64::MIN)),
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        // Fixinig the random error as the order is not quarantee
        let voting_result_account = result
            .get_account(&voting_account)
            .expect("Failed to find funraiser account");

        // Fundraiser should be own by the program id to be able to modify it
        assert_eq!(*voting_result_account.owner(), PROGRAM_ID);

        // Fundraiser should have a length of 81
        assert_eq!(voting_result_account.data().len(), VoteState::LEN);

        // Let's verify the data
        let data = voting_result_account.data();

        // score
        let score_bytes: [u8; 8] = data[0..8]
            .try_into()
            .expect("Expecting 8 bytes for score");
        let score_result = u64::from_le_bytes(score_bytes);
        assert_eq!(score_result, 1u64);
    }

    fn get_voting_account(mollusk: &Mollusk, amount: u64) -> AccountSharedData {
        let mut voting_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(VoteState::LEN),
            VoteState::LEN,
            &PROGRAM_ID,
        );
        voting_account.set_data_from_slice(&[amount.to_le_bytes().to_vec()].concat());

        voting_account
    }
}
