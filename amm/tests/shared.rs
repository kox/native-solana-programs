
use std::mem;

use amm::Config;
use mollusk_svm::Mollusk;
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::AccountState;

pub fn setup() -> (Mollusk, Pubkey) {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));

    let project_name = format!("../target/deploy/{}", env!("CARGO_PKG_NAME"));
    let mut mollusk = Mollusk::new(&program_id, &project_name);

    mollusk_token::token::add_program(&mut mollusk);
    (mollusk, program_id)
}

pub fn create_mint_account(
    mollusk: &Mollusk, 
    authority: Pubkey, 
    supply: u64, 
    decimals: u8, 
    is_initialized: bool,  
    token_program: Pubkey
) -> AccountSharedData {
    let mut account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );

    spl_token::state::Mint {
        mint_authority: COption::Some(authority),
        supply,
        decimals,
        is_initialized,
        freeze_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());

    account
}

pub fn create_token_account(
    mollusk: &Mollusk, 
    mint: Pubkey, 
    owner: Pubkey, 
    amount: u64, 
    token_program_id: Pubkey
) -> AccountSharedData /* (PubKey, AccountSharedData) */ {
    let mut account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
        &token_program_id,
    );
 
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        account.data_as_mut_slice(),
    ).unwrap();

    account
}

pub fn create_config(
    mollusk: &Mollusk,
    status: u8,
    authority: Pubkey,
    mint_x: Pubkey,
    mint_y: Pubkey,
    mint_lp: Pubkey,
    vault_x: Pubkey,
    vault_y: Pubkey,
    fee: u16,
    authority_bump: u8,
    program_id: Pubkey,
) -> AccountSharedData {
    let mut account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(mem::size_of::<Config>()),
        mem::size_of::<Config>(),
        &program_id,
    );
    account.set_data_from_slice(
        &[
            status.to_le_bytes().to_vec(),
            authority.to_bytes().to_vec(),
            mint_x.to_bytes().to_vec(),
            mint_y.to_bytes().to_vec(),
            mint_lp.to_bytes().to_vec(),
            vault_x.to_bytes().to_vec(),
            vault_y.to_bytes().to_vec(),
            fee.to_le_bytes().to_vec(),
            authority_bump.to_le_bytes().to_vec(),
        ]
        .concat(),
    );

    account
}