use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // Burada sizin belirleyeceginiz hata cesitleri olacak.
    // Ornegin, asagidaki custom bir unauthorized hatasi yaratiyor.
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Balance is not enough  can not buy")]
    NotEnoughBalance {},
    #[error("Amount is not enough can not sell")]
    NotEnoughAmount {},
    #[error("Incorrect funds")]
    IncorretFunds {  },
    #[error("cant divide by zero")]
    DivideByZeroError {  },
    #[error("price is not current")]
    PriceNotCurrentError { denom_current: String, denom_provided: String, price_current: Uint128, price_provided: Uint128 },
    #[error("Subtraction underflow")]
    SubtractionError {  },
}
