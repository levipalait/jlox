// Internal dependencies
use super::expression::Expression;
use super::token::Token;

pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Print(Expression),
    Var(Token, Option<Expression>),
    While(Expression, Box<Statement>),
}