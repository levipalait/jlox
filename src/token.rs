use std::fmt::{Debug, Display};

use crate::{literal::Literal, token_type::TokenType};

/// A Token is a piece of String that is parsed from the source code.
/// It gives it it's meaning.
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>, // Literals can be hold directly inside the Token
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            // Since literal can be Some or None, we have to cover both states
            Some(lit) => write!(f, "{} {} {}", self.token_type, self.lexeme, lit),
            None => write!(f, "{} {} None", self.token_type, self.lexeme),
        }
    }
}
