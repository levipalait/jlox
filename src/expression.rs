// External dependencies
use std::fmt::Display;

// Internal dependencies
use crate::token::Token;
use crate::literal::Literal;

#[derive(Debug, PartialEq)]
pub enum Expression { // I <3 Rust enums
    // Non-Terminals

    /// 0: left, 1: operator, 2: right
    Binary(Box<Expression>, Token, Box<Expression>),
    /// 0: expr
    Grouping(Box<Expression>),
    /// 0: operator, 1: right
    Unary(Token, Box<Expression>),

    // Terminals

    /// 0: value
    Literal(Literal),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary(left, operator, right) => write!(f, "({} {} {})", operator.lexeme(), left, right),
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Unary(operator, right) => write!(f, "({} {})", operator.lexeme(), right),
        }
    }
}