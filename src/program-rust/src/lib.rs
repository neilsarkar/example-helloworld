use borsh::{BorshDeserialize, BorshSerialize};
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

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Instruction data: {:?}", _instruction_data);

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let payer_account = next_account_info(accounts_iter)?;

    let receiver_account = next_account_info(accounts_iter)?;

    let program_account = next_account_info(accounts_iter)?;

    let system_account = next_account_info(accounts_iter)?;

    msg!("payer: {} receiver: {} program: {} system: {} system: {}", payer_account.key, receiver_account.key, program_account.key, system_account.key, system_account.key);

    // The account must be owned by the program in order to modify its data
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if program_account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    if system_account.key.ne(&SYSTEM_PROGRAM_ID) {
        msg!("System account not specified as fourth account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // to get amount from instruction data:
    //
    // let amount = input
    //     .get(..8)
    //     .and_then(|slice| slice.try_into().ok())
    //     .map(u64::from_le_bytes)
    //     .ok_or(ProgramError::InvalidInstructionData)?;

    let amount = 1_000_000_000;
    invoke(
        &transfer(payer_account.key, receiver_account.key, amount),
        &[payer_account.clone(), receiver_account.clone()]
    )?;

    msg!("transfer {} lamports from {:?} to {:?}: done", amount, payer_account.key, receiver_account.key);

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&program_account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut program_account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
