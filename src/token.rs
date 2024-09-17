// External dependencies
use std::fmt::{Debug, Display};

// Internal dependencies
use crate::value::Value;

/// A Token is a piece of String that is parsed from the source code.
/// It gives it it's meaning.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Value>, // Literals can be hold directly inside the Token
    line: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Value>,
        line: u32,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // Field access functions
    
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn literal(&self) -> Option<Value> {
        self.literal.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            // Since literal can be Some or None, we have to cover both states
            Some(lit) => write!(f, "{} {} {} {}", self.token_type, self.lexeme, lit, self.line),
            None => write!(f, "{} {} None {}", self.token_type, self.lexeme, self.line),
        }
    }
}

/// All the different token types that a `Token` could possibly have
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    //Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // End of file
    Eof,
}

// We just use the Debug representation when displaying the TokenType
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
