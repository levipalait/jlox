// External dependencies
use anyhow::Result;

// Internal dependencies
use crate::obj::statement::Statement;
use crate::obj::expression::Expression;
use crate::obj::environment::Environment;
use crate::obj::value::Value;
use crate::obj::token::Token;
use crate::obj::token_type::TokenType;
use crate::errors::RuntimeError;

/// Only public function of the interpreter module. Takes in a collection
/// of statements from the outside and interprets them one by one.
/// It does this by creating an Interpreter instance which hosts the
/// environment for storing variables.
pub fn interpret(statements: Vec<Statement>) -> Result<()> {
    let interpreter = Interpreter::new();
    interpreter.interpret(statements)
}

/// Contraption that stores the currently used environment
struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    // going brr
    fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    /// Takes in a collection of statements and executes them.
    /// Also consumes the interpreter because when the interpretation
    /// is done, the program exits (obviously)
    fn interpret(mut self, statements: Vec<Statement>) -> Result<()> {
        for stmt in statements {
            self.execute_statement(&stmt)?;
        }
        Ok(())
    }
}

impl Interpreter {
    /// Takes in a reference to a Statement and executes it based on it's type.
    /// Also calls statement executions an expression evaluations recursively,
    /// by passing the references to linked statements and expressions
    fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Block(stmts) => {
                let prev_env = self.environment.clone(); // Cloning here because readability first
                self.environment = Environment::new_enclosed(prev_env.clone()); // Also cloning here
                let result: Result<()> = (|| {              // When error, don't propagate immediately, because
                    for stmt in stmts {                     // the environment first has to be set back to the
                        self.execute_statement(stmt)?;      // previous one.
                    }
                    Ok(())
                })();
                self.environment = prev_env;    // Set environment back to previous
                result?                         // Propagate error, if there is one
            },
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            },
            Statement::If(cond, then, els) => {
                if is_truthy(self.evaluate_expression(cond)?) { // If truthy, run the then part
                    self.execute_statement(then)?;
                } else if let Some(stmt) = els { // If there is an else clause, run that
                    self.execute_statement(stmt)?;
                }
            },
            Statement::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
            },
            Statement::Var(name, init) => {
                let value = if let Some(expr) = init {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Nil
                };
                self.environment.define(name.lexeme(), value);
            },
        };
        Ok(())
    }

    /// Takes in a reference to an Expression and evaluates it based on it's type.
    /// Makes recursive calls to other expression evaluations.
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Assign(name, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.environment.assign(name.clone(), value.clone())?; // Clone tokens
                Ok(value)
            },
            Expression::Binary(left, op, right) => self.handle_binary(left, op.clone(), right),
            Expression::Grouping(expr) => self.evaluate_expression(expr),
            Expression::Literal(val) => Ok(val.to_owned()),
            Expression::Logical(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                if op.token_type() == TokenType::Or {
                    if is_truthy(left_val.clone()) { // Cloning unnecessary, but idc.
                        return Ok(left_val);
                    }
                } else {
                    if !is_truthy(left_val.clone()) {
                        return Ok(left_val);
                    }
                }

                self.evaluate_expression(right)
            }
            Expression::Unary(op, right) => self.handle_unary(op.clone(), right),
            Expression::Variable(name) => self.environment.get(name.clone()),
        }
    }

    /// Outsourced binary expression evaluation. Takes in borrows, not Box'es
    fn handle_binary(&mut self, left: &Expression, operator: Token, right: &Expression) -> Result<Value> {
        let left_val = self.evaluate_expression(left)?;
        let right_val = self.evaluate_expression(right)?;
    
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
                        return Ok(Value::String(left_str + &right_str));
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

    fn handle_unary(&mut self, operator: Token, right: &Expression) -> Result<Value> {
        let right_val = self.evaluate_expression(right)?;
    
        match operator.token_type() {
            TokenType::Minus => Ok(Value::Number(-get_number_operand(right_val)?)), // Negation of a number
            TokenType::Bang => Ok(Value::Bool(!is_truthy(right_val))), // Negation of a boolean expression
            _ => Err(RuntimeError::Unknown.into()), // Shouldn't be reached :)
        }
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