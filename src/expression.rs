// External dependencies
use anyhow::Result;
use std::fmt::Display;

// Internal dependencies
use crate::errors::RuntimeError;
use crate::token::Token;
use crate::token::TokenType;
use crate::value::Value;

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
    /// 0: left, 1: operator, 2: right
    Binary(Box<Expression>, Token, Box<Expression>),
    /// 0: expr
    Grouping(Box<Expression>),
    /// 0: operator, 1: right
    Unary(Token, Box<Expression>),

    // Terminals
    /// 0: value
    Literal(Value),
}

impl Expression {
    /// Interprets an expression and all of it's sub-expressions
    /// and returns the computed value to the caller
    pub fn interpret(self) -> Result<Value> {
        match self {
            Self::Literal(val) => Ok(val),
            Self::Grouping(expr) => expr.interpret(),
            Self::Unary(op, right) => handle_unary(op, right),
            Self::Binary(left, op, right) => handle_binary(left, op, right),
        }
    }
}

/// Handles unary expression interpreting
fn handle_unary(operator: Token, right: Box<Expression>) -> Result<Value> {
    let right_val = right.interpret()?;

    match operator.token_type() {
        TokenType::Minus => Ok(Value::Number(-get_number_operand(right_val)?)), // Negation of a number
        TokenType::Bang => Ok(Value::Bool(!is_truthy(right_val))), // Negation of a boolean expression
        _ => Err(RuntimeError::Unknown.into()), // Shouldn't be reached :)
    }
}

/// Handles binary expression interpreting
fn handle_binary(left: Box<Expression>, operator: Token, right: Box<Expression>) -> Result<Value> {
    let left_val = left.interpret()?;
    let right_val = right.interpret()?;

    match operator.token_type() {
        // Arithmetic binary expressions
        TokenType::Minus => Ok(Value::Number(
            get_number_operand(left_val)? - get_number_operand(right_val)?, // Subtraction
        )),
        TokenType::Slash => Ok(Value::Number(
            get_number_operand(left_val)? / get_number_operand(right_val)?, // Division
        )),
        TokenType::Star => Ok(Value::Number(
            get_number_operand(left_val)? * get_number_operand(right_val)?, // Multiplication
        )),
        TokenType::Plus => {
            // If both expressions (left and right) are numbers, we want an addition
            if let Value::Number(left_num) = left_val {
                if let Value::Number(right_num) = right_val {
                    return Ok(Value::Number(left_num + right_num));
                }
            }
            // If both are strings, we want a string concatenation
            if let Value::String(left_str) = left_val {
                if let Value::String(right_str) = right_val {
                    return Ok(Value::String(String::from(left_str + &right_str)));
                }
            }
            // If both don't match up, we want an error
            Err(RuntimeError::IncompatibleTypes.into())
        }

        // Comparison binary expressions
        TokenType::Greater => Ok(Value::Bool(
            get_number_operand(left_val)? > get_number_operand(right_val)?, // Greater
        )),
        TokenType::GreaterEqual => Ok(Value::Bool(
            get_number_operand(left_val)? >= get_number_operand(right_val)?, // Greater or Equal
        )),
        TokenType::Less => Ok(Value::Bool(
            get_number_operand(left_val)? < get_number_operand(right_val)?, // Less than
        )),
        TokenType::LessEqual => Ok(Value::Bool(
            get_number_operand(left_val)? <= get_number_operand(right_val)?, // Less than or Equal
        )),

        // Equality binary expressions
        TokenType::BangEqual => Ok(Value::Bool(!is_equal(left_val, right_val))), // Not equal
        TokenType::EqualEqual => Ok(Value::Bool(is_equal(left_val, right_val))), // Equal

        _ => Err(RuntimeError::Unknown.into()), // Shouldn't be reached :)
    }
}

/// Checks if a value is *truthy*
fn is_truthy(value: Value) -> bool {
    !(value == Value::Nil || value == Value::Bool(false))
}

/// Checks if two values are *equal* to eachother.
/// Works seamlessly because Value derives the
/// `PartialEq` trait.
fn is_equal(first: Value, second: Value) -> bool {
    if first == Value::Nil && second == Value::Nil {
        return true;
    }
    if first == Value::Nil {
        return false;
    }

    first == second
}

/// Checks if the given value is a Number value and if so,
/// it returns it
fn get_number_operand(value: Value) -> Result<f64> {
    match value {
        Value::Number(num) => Ok(num),
        _ => Err(RuntimeError::NumberOperand.into()),
    }
}


impl Display for Expression { // recursive printing of expressions
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme(), left, right)
            }
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Unary(operator, right) => write!(f, "({} {})", operator.lexeme(), right),
        }
    }
}
