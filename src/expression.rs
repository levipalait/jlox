// External dependencies
use std::fmt::Display;

// Internal dependencies
use crate::token::Token;
use crate::literal::Literal;

pub enum Expression {
    // Non-Terminals

    /// 0: left, 1: operator, 2: right
    Binary(Box<Expression>, Token, Box<Expression>),
    /// 0: expr
    Grouping(Box<Expression>),
    /// 0: operator, 1: right
    Unary(Token, Box<Expression>),

    // Terminals

    /// 0: value
    Primary(Literal),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary(left, operator, right) => write!(f, "({} {} {})", operator.lexeme(), left, right),
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::Primary(value) => write!(f, "{}", value),
            Expression::Unary(operator, right) => write!(f, "({} {})", operator.lexeme(), right),
        }
    }
}