// External dependencies
use thiserror::Error;

/// This Error type can be used whenever there are Errors
/// regarding command line arguments.
#[derive(Debug, Error)]
pub enum ArgumentError {
    #[error("Argument Error: Invalid Arguments. Usage: jlox [script path]")]
    InvalidArgs,
    #[error("Argument Error: Cannot access command line arguments!")]
    ArgAccessError,
}

/// Whenever there are Errors during the scanning phase,
/// this Error type can be used.
#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Scan Error: At least 1 error occurred while scanning. Aborted!")]
    HadError,
    #[error("Scan Error: Cannot access source code character on line {0}")]
    /// 0: line number
    CharacterAccessError(usize),
    #[error("Scan Error: Unexpected character {0} on line {1}")]
    /// 0: unexpected character, 1: line number
    UnexpectedCharacter(char, usize),
    #[error("Scan Error: Unterminated string on line {0}")]
    /// 0: line number
    UnterminatedString(usize),
}