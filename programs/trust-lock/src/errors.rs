use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not approved")]
    NotApproved,

    #[msg("Token Not Supported")]
    TokenNotSupported,

    #[msg("Overflow")]
    OverflowError,

    #[msg("Underflow")]
    UnderflowError,

    #[msg("Insufficient Funds")]
    InsufficientFunds,
}
