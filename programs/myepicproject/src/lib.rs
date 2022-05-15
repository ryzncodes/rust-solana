use anchor_lang::prelude::*; //import anchor tools

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); //anchor generated program id

#[program] //macros - to call solana via fetch request
pub mod myepicproject { //module
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> { //call context to output result
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {}