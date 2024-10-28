use five8_const::decode_32_const;
use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
    entrypoint
};
use solana_nostd_sha256::hashv;

// Define a 32-byte program identifier by decoding a constant string into a byte array
const ID: [u8; 32] = decode_32_const("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

// Constant marker for use in generating the Program Derived Address (PDA)
const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

// Register `withdraw` as the entry point for this program
entrypoint!(withdraw);

/// # Withdraw Function
///
/// This function allows for the withdrawal of lamports from a Program Derived Address (PDA).
/// It transfers lamports from a vault PDA account to a signer account, if the conditions are met.
///
/// ## Parameters
/// - `_program_id`: The public key of the program.
/// - `accounts`: An array containing `signer` and `vault` accounts.
/// - `data`: A byte array containing the withdrawal amount (first 8 bytes) and a `bump` byte.
///
/// ## Errors
/// - `ProgramError::NotEnoughAccountKeys` if there are not enough accounts.
/// - A panic if `signer` has not signed or if there’s a mismatch in calculated PDA.
pub fn withdraw(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Expect two accounts: `signer` (the one initiating the withdrawal) and `vault` (PDA holding lamports)
    let [signer, vault] = accounts else {
        // Error if insufficient accounts are provided
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Security Check 1: Verify that the signer is an authorized signer
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Parse withdrawal amount directly from `data` pointer as a u64 (unsafe usage)
    let lamports: u64 = unsafe { *(data.as_ptr() as *const u64) };

    // Parse the bump seed from the data array, used for PDA calculation
    let bump = data[8];

    // Calculate the expected PDA using the signer public key, bump seed, program ID, and marker
    let pda = hashv(&[
        signer.key().as_ref(),
        &[bump],
        ID.as_ref(),
        PDA_MARKER,
    ]);

    // Security Check 2: Assert that the computed PDA matches the `vault` key
    assert_eq!(&pda, vault.key().as_ref());

    // Unsafe manipulation of lamports: Modify the balances for `vault` and `signer`
    unsafe {
        *vault.borrow_mut_lamports_unchecked() -= lamports;
        *signer.borrow_mut_lamports_unchecked() += lamports;
    }

    Ok(())
}