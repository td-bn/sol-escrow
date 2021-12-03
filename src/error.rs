use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,

  /// Not rent exempt
  #[error("Not rent exempt")]
  NotRentExempt
}

impl From::<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}