#[path = "./shared.rs"]
mod shared;

#[cfg(test)]
mod initialize_tests {
    use crate::shared;
    use amm::Config;
    
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;


    #[test]
    fn initialize() {
        let (mollusk, program_id) = shared::setup();

        let config = Pubkey::new_unique();

        let data = [
            vec![0],                                // Instruction
            vec![0],                                // status
            Pubkey::default().to_bytes().to_vec(),  // authority
            Pubkey::default().to_bytes().to_vec(),  // mint x
            Pubkey::default().to_bytes().to_vec(),  // mint y
            Pubkey::default().to_bytes().to_vec(),  // mint lp
            Pubkey::default().to_bytes().to_vec(),  // vault x
            Pubkey::default().to_bytes().to_vec(),  // vault y    
            u16::MAX.to_le_bytes().to_vec(),        // fee
            u8::MAX.to_le_bytes().to_vec(),         // authority bump
        ]
        .concat();


        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(config, true),
            ],
        );

        let lamports = mollusk.sysvars.rent.minimum_balance(Config::LEN);

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (config, AccountSharedData::new(lamports, Config::LEN, &program_id)),
            ],
            &[Check::success()]
        );
    }
}
