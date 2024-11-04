#[cfg(test)]
mod tests {
    use std::mem;

    use fundraiser::Fundraiser;

    use mollusk_svm::{ program, Mollusk };
    use solana_sdk::{account::{AccountSharedData, ReadableAccount}, account_info::AccountInfo, clock::Slot, deserialize_utils, instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

    #[test]
    fn initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222"
        ));

        let mollusk = Mollusk::new(
            &program_id,
            "../target/deploy/fundraiser"
        );

        // let initializer = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        // 200 slots until fundraising will end
        let slot = mollusk.sysvars.clock.slot + 200;

        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let (fundraiser_pda, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

        let data = [
            vec![0],
            mint.to_bytes().to_vec(),               // mint
            100_000_000u64.to_le_bytes().to_vec(),  // reamining_amount
            slot.to_le_bytes().to_vec(),            // slot target
            1u8.to_le_bytes().to_vec()              // bump (i'm hesitating about it as the acocunt is created aside)

            // 100u64.to_le_bytes().to_vec(),
            /* maker_ta_b.to_bytes().to_vec(),
            mint_a.to_bytes().to_vec(),
            mint_b.to_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(), */
        ].concat();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, true), // It should be a signer because this account shouldn't exist yet
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let lamports = mollusk.sysvars.rent.minimum_balance(81);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (fundraiser, AccountSharedData::new(lamports, 81, &program_id)),
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());

        // Fixinig the random error as the order is not quarantee
        let funraiser_result_account = result.get_account(&fundraiser).expect("Failed to find funraiser account");

        assert_eq!(funraiser_result_account.data().len(), 81);

        let data = funraiser_result_account.data();

        let pubkey_bytes: [u8; 32] = data[0..32].try_into().expect("Expected 32 bytes for pubkey");
        let maker_pubkey = Pubkey::from(pubkey_bytes);

        assert_eq!(maker_pubkey.to_string(), maker.to_string());

        let mint_bytes: [u8; 32]  = data[32..64].try_into().expect("Expecting 8 bytes for mint");
        let mint_pubkey = Pubkey::from(mint_bytes);

        assert_eq!(mint_pubkey.to_string(), mint.to_string());

        let remaining_amount_bytes: [u8; 8]  = data[64..72].try_into().expect("Expecting 8 bytes for remaining_amount");
        let remaining_amount_result = u64::from_le_bytes(remaining_amount_bytes);

        assert_eq!(remaining_amount_result, 100_000_000u64);

        let slot_bytes: [u8; 8]  = data[72..80].try_into().expect("Expecting 8 bytes for slot");
        let slot_result = u64::from_le_bytes(slot_bytes);

        assert_eq!(slot_result, slot);

        /* Not using the bump yet
        let bump_bytes: [u8; 1]  = data[80..81].try_into().expect("Expecting 1 bytes for bump");
        let bump_result = u8::from_le_bytes(bump_bytes);

        assert_eq!(bump_result, bump); 
        */
    }
}