use std::fmt::Display;

// Internal dependencies
use super::expression::Expression;
use super::token::Token;

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Print(Expression),
    Var(Token, Option<Expression>),
    While(Expression, Box<Statement>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Block(vec) => {
                write!(f, "(block ")?;
                for stmt in vec {
                    write!(f, "({}), ", stmt)?;
                }
                write!(f, ")")?;
                Ok(())
            },
            Statement::Expression(expr) => write!(f, "(expr_stmt {})", expr),
            Statement::If(cond, then, els) => write!(f, "(if {} then {} else {:?})", cond, then, els),
            Statement::Print(expr) => write!(f, "(print {})", expr),
            Statement::Var(name, expr) => write!(f, "(var {} = {:?})", name.lexeme(), expr),
            Statement::While(cond, stmt) => write!(f, "(while {} do {})", cond, stmt),
        }
    }
}