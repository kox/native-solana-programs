#[cfg(test)]
mod tests {
    use std::mem;

    use fundraiser::Fundraiser;

    use mollusk_svm::{ program, Mollusk };
    use solana_sdk::{account::AccountSharedData, account_info::AccountInfo, clock::Slot, deserialize_utils, instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

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

        let (fundraiser_pub, fundraiser_account) = result.resulting_accounts.get(1).unwrap();

        assert_eq!(fundraiser_pub, &fundraiser);
        // let's deserialize

        let fundraiser_data = Fundraiser::from_account_info_unchecked(fundraiser_account);

        // let data: Fundraiser = fundraiser_account.deserialize_data().unwrap();
        
        println!("hello data");
        /* let fundraiser_account_info = AccountInfo::from(fundraiser_account);

        let deserialize_fundraiser = Fundraiser::from_account_info_unchecked(&fundraiser_account_info);
        assert_eq!(deserialized_fundraiser.maker(), maker); */

        // let data = fundraiser_account.data();
        

        // New step: Read and validate the data in the fundraiser account
        /* let fundraiser_account = mollusk.get_account(&fundraiser).unwrap();
        let fundraiser_data = fundraiser_account.data(); */



    }
}