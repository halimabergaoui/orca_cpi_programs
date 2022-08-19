
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use anchor_lang::prelude::*;


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
   
    // Get the account to say hello to
    let program = next_account_info(accounts_iter)?;
    let position=next_account_info(accounts_iter)?;
    let position_mint=next_account_info(accounts_iter)?;
    let position_token_account=next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;
    let funder=next_account_info(accounts_iter)?;
    let owner=next_account_info(accounts_iter)?;
    let whirlpool=next_account_info(accounts_iter)?;
    let system_program=next_account_info(accounts_iter)?;
    let rent=next_account_info(accounts_iter)?;
    let associated_token_program=next_account_info(accounts_iter)?;



    let accounts = whirlpool::cpi::accounts::OpenPosition {
        position: position.clone(),
        position_mint: position_mint.clone(),
        position_token_account: position_token_account.clone(),        
        token_program: token_program.clone(),
        funder: funder.clone(),
        owner: owner.clone(),
        whirlpool: whirlpool.clone(),
        system_program: system_program.clone(),
        rent: rent.clone(),
        associated_token_program: associated_token_program.clone(),
    };
    let position_bump = instruction_data[0];
    let tick_lower_index=0;
    let tick_upper_index=128;
    let ctx =CpiContext::new(program.clone(), accounts);
whirlpool::cpi::open_position(ctx,whirlpool::state::OpenPositionBumps { position_bump: position_bump },tick_lower_index,tick_upper_index);   

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
