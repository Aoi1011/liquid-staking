use std::fmt::Display;

use anchor_lang::prelude::*;

#[error_code]
pub enum CommonError {
    #[msg("Wrong reserve owner. Must be a system account.")]
    WrongReserveOwener,
    #[msg("Reserve must have no data, but has data.")]
    NonEmptyReserveData,
    #[msg("Invalid initial reserve lamports")]
    InvalidInitialReserveLamports,
    #[msg("Zero validator chunk size")]
    ZeroValidatorChunkSize,
    #[msg("Too big validator chunk size")]
    TooBigValidatorChunkSize,
    #[msg("Zero credit chunk size")]
    ZeroCreditChunkSize,
    #[msg("Too big credit chunk size")]
    TooBigCreditChunkSize,
    #[msg("Too low credit fee")]
    TooLowCreditFee,
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    #[msg("Non empty initial mint supply")]
    MintHasInitialSupply,
    #[msg("Invalid owner fee state")]
    InvalidOwnerFeeState,

    #[msg("1910 Invalid program id. For using program from another account please update in the code.")]
    InvalidProgramId = 6116,

    #[msg("FFA0 Unexpected account")]
    UncheckedAccount = 65140,

    #[msg("CACF Calculation failure")]
    CalculationFailure = 51619,

    #[msg("B3AA You can't deposit a stake-account with lockup")]
    AccountWithLockup = 45694,

    #[msg("2000 Number too low")]
    NumberTooLow = 7892,
    #[msg("2001 Number to high")]
    NumberTooHigh = 7893,

    #[msg("1100 Fee too high")]
    FeeTooHigh = 4052,

    #[msg("1101 Min fee > Max fee")]
    FeeWrongWayRound = 4053,

    #[msg("1102 Liquidity target too low")]
    LiquidityTargetTooLow = 4054,

    #[msg("1103 Ticket not due. Wait more epochs")]
    TicketNotDue = 4055,

    #[msg("1104 Ticket not ready. Wait a few hours and try again")]
    TicketNotReady = 4056,

    #[msg("1105 Wrong Ticket beneficiary")]
    WrongBeneficiary = 4057,

    #[msg("1199 Insufficient Liquidity in the liquidity pool")]
    InsufficientLiquidity = 4205,

    #[msg("BAD1 Invalid validator")]
    InvalidValidator = 47525,
}

// Conversion into a [cmpError]
pub trait IntoCmpError {
    /// Converts the value into a [CmpError].
    fn into_cmp_error(self) -> Option<CmpError>;
}

impl<T> IntoCmpError for Result<T> {
    fn into_cmp_error(self) -> Option<CmpError> {
        self.err()?.into_cmp_error()
    }
}

impl IntoCmpError for anchor_lang::error::Error {
    fn into_cmp_error(self) -> Option<CmpError> {
        Some(CmpError(self))
    }
}

impl IntoCmpError for Option<anchor_lang::error::Error> {
    fn into_cmp_error(self) -> Option<CmpError> {
        self?.into_cmp_error()
    }
}

impl From<anchor_lang::error::Error> for CmpError {
    fn from(err: anchor_lang::error::Error) -> Self {
        CmpError(err)
    }
}

// impl From<CommonError> for ProgramError {
//     fn from(err: CommonError) -> Self {

//     }
// }

/// A Comparable error: an error that can be compared (via equality) to other errors.
#[repr(transparent)]
#[derive(Debug)]
pub struct CmpError(pub anchor_lang::error::Error);

impl PartialEq for CmpError {
    fn eq(&self, other: &Self) -> bool {
        let (CmpError(a), CmpError(b)) = (self, other);
        match (a, b) {
            (Error::AnchorError(err_a), Error::AnchorError(err_b)) => {
                err_a.error_code_number == err_b.error_code_number
            }
            (Error::ProgramError(err_a), Error::ProgramError(err_b)) => {
                err_a.program_error == err_b.program_error
            }
            _ => false,
        }
    }
}

impl Eq for CmpError {}

impl Display for CmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
