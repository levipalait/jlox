// External dependencies
use anyhow::Result;
use std::collections::{hash_map, HashMap};

// Internal dependencies
use crate::RuntimeError;
use super::value::Value;
use super::token::Token;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {

    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value> {
        if let Some(inner) = self.values.get(&name.lexeme()).cloned() {
            return Ok(inner);
        }
        if let Some(encl) = &self.enclosing {
            return encl.get(name);
        }

        Err(RuntimeError::UndefinedVariable.into())
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<()> {
        if let hash_map::Entry::Occupied(mut entry) = self.values.entry(name.lexeme()) {
            entry.insert(value);
            return Ok(());
        }
        if let Some(encl) = &mut self.enclosing {
            encl.assign(name, value)?;
            return Ok(());
        }

        Err(RuntimeError::UndefinedVariable.into())
    }
}