// External dependencies
use std::fmt::Display;

/// There are two different literal types: String literals and Number literals.
/// Those can be represented using the Literal enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s), // just the string
            Self::Number(n) => write!(f, "{}", n), // just the number
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}