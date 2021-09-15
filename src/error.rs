use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Account Already Initialized")]
    AccountAlreadyInitialized,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
}

impl From<EscrowError> for ProgramError {
    fn from(err: EscrowError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
