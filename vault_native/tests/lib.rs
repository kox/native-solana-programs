#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount}, instruction::{AccountMeta, Instruction}, pubkey::Pubkey
    };

    #[test]
    fn withdraw() {
        // Define the program ID as a constant Pubkey, same as in the main program
        let program_id = Pubkey::new_from_array([
            0x7b, 0x07, 0x5a, 0x4f, 0xca, 0x15, 0x61, 0x6e, 
            0xbe, 0x53, 0xc1, 0xa8, 0x43, 0x6f, 0x42, 0x89, 
            0x2b, 0x02, 0x1a, 0xb6, 0x62, 0x5a, 0x2a, 0x02, 
            0x2a, 0x68, 0x9a, 0xef, 0xbd, 0xed, 0x26, 0xef
        ]);

        println!("{}", program_id.to_string());

        // Create a unique `signer` Pubkey and calculate the PDA `vault` based on the `signer` and `program_id`
        let signer = Pubkey::new_unique();
        let (vault, bump) =
            Pubkey::try_find_program_address(&[signer.as_ref()], &program_id).unwrap();

        // Create the withdraw instruction with specified lamports and bump for PDA
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&1_000_000_000u64.to_le_bytes()[..], &[bump]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
            ],
        );

        // Initialize the Mollusk virtual machine for testing, loading the program binary
        let mollusk = Mollusk::new(&program_id, "../target/deploy/vault_native");

        // Process the withdraw instruction with mock account data for `signer` and `vault`
        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    signer,
                    AccountSharedData::new(0, 0, &Pubkey::default()),
                ),
                (vault, AccountSharedData::new(1_000_000_000u64, 0, &program_id)),
            ],
        );

        // Verify the final lamports balances after withdrawal:
        // - The `signer` should now hold the withdrawn amount (1,000,000,000 lamports)
        // - The `vault` should be empty, having transferred all lamports to `signer`
        assert_eq!(result.get_account(&signer).unwrap().lamports(), 1_000_000_000);
        assert_eq!(result.get_account(&vault).unwrap().lamports(), 0);

        // Ensure that the instruction executed successfully without errors
        assert!(!result.program_result.is_err());
    }
}