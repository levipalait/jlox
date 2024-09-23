// Internal dependencies
use super::expression::Expression;
use super::token::Token;

pub enum Statement {
    Block(Vec<Statement>),
    Print(Expression),
    Expression(Expression),
    Var(Token, Option<Expression>),
}