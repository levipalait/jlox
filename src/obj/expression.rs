// External dependencies
use std::fmt::Display;

// Internal dependencies
use super::token::Token;
use super::value::Value;

/// The Expression is a construct, that holds a linked
/// AST of non-terminal expressions and terminal expressions.
/// Non-terminal expressions can hold other expressions, which
/// finally hold terminal expressions at the leaves of the tree.
/// The expression tree (AST) can be traversed recursively to
/// produce values.
#[derive(Debug, PartialEq)]
pub enum Expression {
    // I <3 Rust enums
    // Non-Terminals
    /// 0: name, 1: expression
    Assign(Token, Box<Expression>),
    /// 0: left, 1: operator, 2: right
    Binary(Box<Expression>, Token, Box<Expression>),
    /// 0: expr
    Grouping(Box<Expression>),
    /// 0: left, 1: operator, 2: right
    Logical(Box<Expression>, Token, Box<Expression>),
    /// 0: operator, 1: right
    Unary(Token, Box<Expression>),

    // Terminals
    /// 0: value
    Literal(Value),
    /// 0: name
    Variable(Token),
}

impl Display for Expression { // recursive printing of expressions
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme(), left, right)
            }
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::Literal(val) => write!(f, "{}", val),
            Expression::Unary(op, right) => write!(f, "({} {})", op.lexeme(), right),
            Expression::Variable(name) => write!(f, "(var {})", name.lexeme()),
            Expression::Assign(name, expr) => write!(f, "(= {} {})", name.lexeme(), expr),
            Expression::Logical(left, op, right) => write!(f, "(logical {} {} {})", left, op.lexeme(), right),
        }
    }
}
