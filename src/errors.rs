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
    #[error("Parse Error: Unterminated grouping on line {0}")]
    UnterminatedGrouping(u32),
    #[error("Parse Error: Unterminated print statement on line {0}")]
    UnterminatedPrintStatement(u32),
    #[error("Parse Error: Unterminated expression statement on line {0}")]
    UnterminatedExpressionStatement(u32),
    #[error("Parse Error: Unterminated variable declaration on line {0}")]
    UnterminatedVarDeclaration(u32),
    #[error("Parse Error: Unterminated block on line {0}")]
    UnterminatedBlock(u32),
    #[error("Parse Error: Expected identifier on line {0}")]
    ExpectedIdentifier(u32),
    #[error("Parse Error: Expected opening parentheses \"(\" on line {0}")]
    ExprectedLeftParen(u32),
    #[error("Parse Error: Exprected closing parentheses \")\" on line {0}")]
    ExpectedRightParen(u32),
    /// 0: Current token formatted as String. Should use `format!` for that
    #[error("Parse Error: Expected expression on line {0}")]
    ExpectedExpression(u32),
    #[error("Parse Error: Expected semicolon on line {0}")]
    ExpectedSemicolon(u32),
    #[error("Parse Error: Expected literal on line {0}")]
    /// 0: token index
    NoLiteralOnToken(u32),
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