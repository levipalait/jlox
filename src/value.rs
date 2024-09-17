// External dependencies
use std::fmt::Display;

/// There are two different literal types: String literals and Number literals.
/// Those can be represented using the Literal enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s), // just the string
            Self::Number(n) => write!(f, "{}", n), // just the number
            Self::Bool(b) => write!(f, "{}", b),   // just the boolean
            Self::Nil => write!(f, "nil"),
        }
    }
}