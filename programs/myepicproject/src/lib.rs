use anchor_lang::prelude::*; //import anchor tools

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); //anchor generated program id

#[program] //macros - to call solana via fetch request
pub mod myepicproject { //module
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> { //call context to output result
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {

    /*init will tell Solana to create new account based on our program
    payer = user: tells whos paying for the created account.
    9000 bytes (9kb) space for account.  */
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>, //proves to program that user calling owns wallet 
    pub system_program: Program <'info, System>, //program that runs solana
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account] //holds total_gifs variable.
pub struct BaseAccount {
    pub total_gifs: u64,
}