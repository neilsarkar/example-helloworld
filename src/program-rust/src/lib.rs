use std::{convert::TryInto};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    system_instruction::{transfer},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Accounts expected:
// 0. `[signer, writable]` Debit lamports
// 1. `[]`                 System program
// 2. `[writable]`         Credit lamports/n
// n. `[writable]`         Credit lamports/n
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    program_accounts: &[AccountInfo], // The account to say hello to
    input: &[u8],
) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut program_accounts.iter();

    // First account should be signed account of payer
    let payer_account = next_account_info(accounts_iter)?;
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Second account should be system account for transfer
    let system_account = next_account_info(accounts_iter)?;
    if system_account.key.ne(&SYSTEM_PROGRAM_ID) {
        msg!("System account not specified as second account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Collect remaining accounts
    let mut count = 0;
    let mut payee_accounts: Vec<&AccountInfo> = Vec::new();
    loop {
        let account = match next_account_info(accounts_iter) {
            Ok(account_info) => account_info,
            Err(ProgramError::NotEnoughAccountKeys) => break,
            Err(error) => panic!("{}", error),
        };
        payee_accounts.push(account);
        count += 1;
    }
    if count <= 0 || count > 10 {
        msg!("Tried to split between {} accounts, max is 10", count);
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    // parse amount as u64 from 8 little-endian u8s of instruction data
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // for each provided account up to 10, split the amount
    for account in payee_accounts {
        invoke(
            &transfer(payer_account.key, account.key, amount / count),
            &[payer_account.clone(), account.clone()]
        )?;
        msg!("transferred {} lamports from {:?} to {:?}", amount / count, payer_account.key, account.key);
    }

    Ok(())
}