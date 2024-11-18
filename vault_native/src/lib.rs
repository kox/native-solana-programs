use solana_program::entrypoint;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::hashv, program_error::ProgramError,
    pubkey::Pubkey,
};

// Define a constant public key (Pubkey) to represent the unique identifier for this program
const ID: Pubkey = Pubkey::new_from_array([
    0x7b, 0x07, 0x5a, 0x4f, 0xca, 0x15, 0x61, 0x6e, 0xbe, 0x53, 0xc1, 0xa8, 0x43, 0x6f, 0x42, 0x89,
    0x2b, 0x02, 0x1a, 0xb6, 0x62, 0x5a, 0x2a, 0x02, 0x2a, 0x68, 0x9a, 0xef, 0xbd, 0xed, 0x26, 0xef,
]);

// Constant marker string to help generate a Program Derived Address (PDA) unique to this program
const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

// Set the entry point of the Solana program, linking it to the process_instruction function
entrypoint!(process_instruction);

/// # Withdraw
///
/// This function handles withdrawing funds from a PDA that has lamports deposited to it.
///
/// - `_program_id`: A reference to the program's ID.
/// - `accounts`: An array of accounts that the program is given access to for processing.
/// - `data`: The byte array containing instructions and amounts for withdrawal.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Extract the `signer` (authorized user) and `vault` (account holding funds) from `accounts`
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure that `signer` has signed the transaction, meaning they're authorized to withdraw funds
    assert!(signer.is_signer);

    // Parse `lamports` (amount to withdraw) from the first 8 bytes of the `data` array
    let lamports: u64 = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    // Parse the `bump` byte from the 9th byte of the `data` array
    let bump = data[8];

    // Generate the PDA (program-derived address) that points to the `vault`
    let pda = hashv(&[signer.key.as_ref(), &[bump], ID.as_ref(), PDA_MARKER]);

    // Ensure that the generated PDA matches the `vault`'s address; if not, this fails
    assert_eq!(pda.to_bytes(), vault.key.as_ref());

    // Withdraw lamports: decrease the balance in `vault` and increase the balance in `signer`
    **vault.try_borrow_mut_lamports()? -= lamports;
    **signer.try_borrow_mut_lamports()? += lamports;

    // Return success if withdrawal was successful
    Ok(())
}
