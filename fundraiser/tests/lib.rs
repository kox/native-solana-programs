#[cfg(test)]
mod tests {
    use std::mem;

    use mollusk_svm::{ program, Mollusk };
    use solana_sdk::{clock::Slot, pubkey::Pubkey};

    #[test]
    fn initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222"
        ));

        let mollusk = Mollusk::new(
            &program_id,
            "../target/deploy/fundraiser"
        );

        let initializer = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let data = [
            vec![0],
            mint.to_bytes().to_vec(),
            100_000_000u64.to_le_bytes().to_vec(),
            100u64.to_le_bytes().to_vec(),
            /* maker_ta_b.to_bytes().to_vec(),
            mint_a.to_bytes().to_vec(),
            mint_b.to_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(), */
        ]

        

    }
}