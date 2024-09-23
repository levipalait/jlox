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
    /// 0: unexpected character, 1: starting line number
    UnexpectedCharacter(char, usize),
    #[error("Scan Error: Unterminated string starting on line {0}")]
    /// 0: line number
    UnterminatedString(usize),
}

/// This error type can be used whenever there are
/// Errors during the parsing phase
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parse Error: At least 1 error occurred while parsing. Aborted!")]
    HadError,
    #[error("Parse Error: Cannot access token at index {0}")]
    /// 0: token index
    TokenAccessError(usize),
    #[error("Parse Error: Unterminated grouping.")]
    UnterminatedGrouping,
    #[error("Parse Error: Unterminated print statement.")]
    UnterminatedPrintStatement,
    #[error("Parse Error: Unterminated expression statement.")]
    UnterminatedExpressionStatement,
    #[error("Parse Error: Unterminated variable declaration.")]
    UnterminatedVarDeclaration,
    #[error("Parse Error: Unterminated block.")]
    UnterminatedBlock,
    #[error("Parse Error: Expected identifier.")]
    ExpectedIdentifier,
    #[error("Parse Error: Expected expression. Current token: {0}")]
    /// 0: Current token formatted as String. Should use `format!` for that
    ExpectedExpression(String),
    #[error("Parse Error: Expected literal on token {0}")]
    /// 0: token index
    NoLiteralOnToken(usize),
    #[error("Parse Error: Invalid assignment target.")]
    InvalidAssignmentTarget,
}

/// This error type can be used whenever there is
/// an Error during code execution.
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Runtime Error: Operand must be a number.")]
    NumberOperand,
    #[error("Runtime Error: Incompatible types.")]
    IncompatibleTypes,
    #[error("Runtime Error: Undefined variable.")]
    UndefinedVariable,
    #[error("Runtime Error: Unknown error.")]
    Unknown,
}