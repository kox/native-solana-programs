use solana_program::{account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey};

declare_id!("22222222222222222222222222222222222222222222");

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    _accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    
    Ok(())
}
