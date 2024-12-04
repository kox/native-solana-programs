use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};

//
// Publish instruction
//
// A publisher who owns a NFT, wants to sell it in a marketplace
// It will require to pass:
// > publisher
// > publisher_ta
// > the marketplace
// > the token account
// > price
// 
pub fn publish(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // let [publisher, marketplace, , _token_program] = accounts 
    
    Ok(())
}
