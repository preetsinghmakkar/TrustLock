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

    #[msg("Already Pitched")]
    AlreadyPitched,

    #[msg("Order Owner is not Correct")]
    NotValidOwner,

    #[msg("Order Locked - No More Pitching Allowed")]
    OrderLocked,

    #[msg("Invalid Release Time")]
    InvalidReleaseTime,

    #[msg("You are not the Fulfiler of this order")]
    WrongFulfiller,

    #[msg("Not Authorized To Review")]
    NotAuthorizedToReview,
}
