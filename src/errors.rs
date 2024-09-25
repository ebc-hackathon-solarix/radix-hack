use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An error occurred: {0}")]
    GeneralError(String),

    #[error("Price must be greater than zero")]
    ZeroOrNegativePriceError,

    #[error("Total supply must be greater than zero")]
    ZeroOrNegativeTotalSupplyError,

    #[error("Insufficient token amount: expected {expected}, found {found}")]
    InsufficientTokenAmount {
        expected: u32,
        found: u32,
    },

    #[error("Not enough supply. You requested to purchase {requested} but only {available} is available")]
    InsufficientSupply {
        requested: u32,
        available: u32,
    },

    #[error("Not authorized to claim earnings" )]
    NotAuthorizedToClaimEarningsError,

    #[error("Not authorized to claim payout" )]
    NotAuthorizedToClaimPayoutError,

    #[error("Not authorized to deposit earnings" )]
    NotAuthorizedToDepositEarningsError,

}