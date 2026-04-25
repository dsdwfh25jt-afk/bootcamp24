use anchor_lang::prelude::*;

declare_id!("GzAkiWUe58KgD9S7LwefX3UHuaMka5GcGNvPzdckQdAr");

#[program]
pub mod solana_counter {
    use super::*;
   pub fn initialize (ctx : Context<Initialize>) -> Result<()> {
        // initial counter value 
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account initialized ! Current count : {}", counter.count);
        Ok(())
   }

   pub fn increament (ctx : Context<Increament>) -> Result<()> {
        // increament counter function
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented ! New Count : {}",counter.count);
        Ok(())
   }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // validating account 

    #[account(init, payer = user, space = 8+8)]
    pub counter: Account<'info , Couonter>,

    #[account(mut)]

    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increament <'info> {
    pub counter: Account<'info, Couonter>,
}

#[account] 
pub struct Couonter {
    pub count: u64,
}