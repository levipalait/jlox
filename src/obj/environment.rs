// External dependencies
use anyhow::Result;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::rc::Rc;

// Internal dependencies
use crate::RuntimeError;
use super::value::Value;
use super::token::Token;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {

    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define_inner(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value> {
        if let Some(inner) = self.values.get(name.lexeme().as_str()).cloned() {
            return Ok(inner);
        }
        if let Some(encl) = &self.enclosing {
            return encl.borrow().get(name);
        }

        Err(RuntimeError::UndefinedVariable.into())
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<()> {
        if self.values.contains_key(name.lexeme().as_str()) {
            self.values.insert(name.lexeme(), value);
        } else if let Some(encl) = &self.enclosing {
            encl.borrow_mut().assign(name, value)?;
        } else {
            return Err(RuntimeError::UndefinedVariable.into());
        }
        Ok(())
    }
}