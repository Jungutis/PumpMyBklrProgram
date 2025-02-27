use anchor_lang::prelude::*;

declare_id!("8g85kXiS8GytVZFCyvLxZ4P4aZj3HpERmd44dFJbB7ut");

#[program]
pub mod bklr_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
