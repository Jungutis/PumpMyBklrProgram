use crate::constants::INITIAL_LAMPORTS_FOR_POOL;
use crate::constants::INITIAL_PRICE_DIVIDER;
use crate::constants::PROPORTION;
use crate::errors::CustomError;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct CurveConfiguration {
    pub fees: f64,
}

impl CurveConfiguration {
    pub const SEED: &'static str = "CurveConfiguration";
    
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8;

    pub fn new(fees: f64) -> Self {
        Self { fees }
    }
}