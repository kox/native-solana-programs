#[cfg(test)]
mod initialize_tests {
    use fundraiser::Fundraiser;

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
    fn initialize() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "../target/deploy/fundraiser");
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let maker = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        // TODO_ Probably during the initialization, we should create the vault with 0 tokens?
        let (_, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &PROGRAM_ID);

        // Example -> 200 slots campaign
        let slot = mollusk.sysvars.clock.slot + 200;

        let data = [
            vec![0],
            mint.to_bytes().to_vec(),              // mint
            100_000_000u64.to_le_bytes().to_vec(), // remaining_amount
            slot.to_le_bytes().to_vec(),           // slot target
            bump.to_le_bytes().to_vec(),           // authority bump
        ]
        .concat();

        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, true), // It should be a signer because this account shouldn't exist yet
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        // we will create an account rent exempt with enough lamports to cover basic accountinfo plus fundraiser data
        let lamports = mollusk.sysvars.rent.minimum_balance(81);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    fundraiser,
                    AccountSharedData::new(lamports, 81, &PROGRAM_ID),
                ),
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        // Fixinig the random error as the order is not quarantee
        let fundraiser_result_account = result
            .get_account(&fundraiser)
            .expect("Failed to find funraiser account");

        // Fundraiser should be own by the program id to be able to modify it
        assert_eq!(*fundraiser_result_account.owner(), PROGRAM_ID);

        // Fundraiser should have a length of 81
        assert_eq!(fundraiser_result_account.data().len(), Fundraiser::LEN);

        // Let's verify the data
        let data = fundraiser_result_account.data();

        // Maker Pubkey
        let pubkey_bytes: [u8; 32] = data[0..32]
            .try_into()
            .expect("Expected 32 bytes for pubkey");
        let maker_pubkey = Pubkey::from(pubkey_bytes);
        assert_eq!(maker_pubkey.to_string(), maker.to_string());

        // Mint Pubkey
        let mint_bytes: [u8; 32] = data[32..64].try_into().expect("Expecting 8 bytes for mint");
        let mint_pubkey = Pubkey::from(mint_bytes);
        assert_eq!(mint_pubkey.to_string(), mint.to_string());

        // Remaining Amount
        let remaining_amount_bytes: [u8; 8] = data[64..72]
            .try_into()
            .expect("Expecting 8 bytes for remaining_amount");
        let remaining_amount_result = u64::from_le_bytes(remaining_amount_bytes);
        assert_eq!(remaining_amount_result, 100_000_000u64);

        // Slot
        let slot_bytes: [u8; 8] = data[72..80].try_into().expect("Expecting 8 bytes for slot");
        let slot_result = u64::from_le_bytes(slot_bytes);
        assert_eq!(slot_result, slot);

        // authority bump
        let bump_bytes: [u8; 1] = data[80..81].try_into().expect("Expecting 1 bytes for bump");
        let bump_result = u8::from_le_bytes(bump_bytes);

        assert_eq!(bump_result, bump);
    }
}
