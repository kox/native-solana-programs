#[path = "./shared.rs"]
mod shared;

#[cfg(test)]
mod initialize_tests {
    use crate::shared;
    use marketplace_native::Marketplace;

    use mollusk_svm::result::Check;
    use solana_sdk::{
        account::{ AccountSharedData, ReadableAccount },
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn initialize() {
        let (mollusk, program_id) = shared::setup();

        let marker = Pubkey::new_unique();
        let marketplace = Pubkey::new_unique();

        let data = [
            vec![0],                               // Instruction
            marker.to_bytes().to_vec(),            // marker
            u64::MAX.to_le_bytes().to_vec(),       // fee
            u8::MAX.to_le_bytes().to_vec(),        // authority bump
            u8::MAX.to_le_bytes().to_vec(),        // authority bump
        ]
        .concat();

        let instruction =
            Instruction::new_with_bytes(program_id, &data, vec![AccountMeta::new(marketplace, false)]);

        let lamports = mollusk.sysvars.rent.minimum_balance(Marketplace::LEN);

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &vec![(
                marketplace,
                AccountSharedData::new(lamports, Marketplace::LEN, &program_id),
            )],
            &[Check::success()],
        );

        assert!(!result.program_result.is_err());

        // We could add some tests to the config account created
        let marketplace_result_account = result
            .get_account(&marketplace)
            .expect("Failed to find marketplace account");

        // Fundraiser should be own by the program id to be able to modify it
        assert_eq!(*marketplace_result_account.owner(), program_id);

        // Fundraiser should have a length of 81
        assert_eq!(marketplace_result_account.data().len(), Marketplace::LEN);

        // Let's verify the data
        /*
        let data = marketplace_result_account.data();

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

        assert_eq!(bump_result, bump); */
    }
}
