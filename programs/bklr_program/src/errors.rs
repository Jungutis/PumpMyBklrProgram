use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid fee")]
    InvalidFee,
}