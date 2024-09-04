use thiserror::Error;

/// This Error type can be used whenever there are Errors
/// regarding command line arguments.
#[derive(Debug, Error)]
pub enum ArgumentError {
    #[error("Invalid Arguments. Usage: jlox [script path]")]
    InvalidArgs,
    #[error("Cannot access command line arguments!")]
    ArgAccessError,
}

/// Whenever there are Errors during the scanning phase,
/// this Error type can be used.
#[derive(Debug, Error)]
pub enum ScanError {
    #[error("At least 1 error occurred while scanning. Aborted!")]
    HadError,
    #[error("Cannot access source code character {current} on line {line}")]
    CharacterAccessError {
        current: usize,
        line: usize,
    },
    #[error("Unexpected character on line {line}")]
    UnexpectedCharacter {
        line: usize,
    }
}