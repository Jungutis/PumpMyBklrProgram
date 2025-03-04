use anchor_lang::prelude::*;

declare_id!("8g85kXiS8GytVZFCyvLxZ4P4aZj3HpERmd44dFJbB7ut");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod constants;

use crate::instructions::*;

#[program]
pub mod bklr_program {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCurveConfiguration>, fee:f64) -> Result<()> {
        instructions::initialize(ctx,fee)
    }
}
