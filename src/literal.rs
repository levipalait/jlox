use std::fmt::Display;

/// There are two different literal types: String literals and Number literals.
/// Those can be represented using the Literal enum.
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}