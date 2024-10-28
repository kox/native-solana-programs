use five8_const::decode_32_const;
use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
    entrypoint
};
use solana_nostd_sha256::hashv;

const ID: [u8; 32] = decode_32_const("9HFegTZnvebYjf9kSa6k3WBm93hRfogWB5B1goUrq1oL");

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(withdraw);

/// # Withdraw
///
/// Handles withdrawing funds from a PDA that has previously had lamports deposited to it.
pub fn withdraw(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(signer.is_signer());

    let lamports: u64 = unsafe { *(data.as_ptr() as *const u64) };
    let bump = data[8];
    let pda = hashv(&[
        signer.key().as_ref(),
        &[bump],
        ID.as_ref(),
        PDA_MARKER,
    ]);

    assert_eq!(&pda, vault.key().as_ref());

    unsafe {
        *vault.borrow_mut_lamports_unchecked() -= lamports;
        *signer.borrow_mut_lamports_unchecked() += lamports;
    }
    Ok(())
}