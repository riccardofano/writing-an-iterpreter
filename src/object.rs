use std::{collections::HashMap, fmt::Display};

use crate::{
    ast::{Expression, Identifier, Statement},
    evaluator::Env,
};

pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Error(String),
    Boolean(bool),
    Integer(i64),
    ReturnValue(Box<Object>),
    Function(Vec<Expression>, Statement, Env),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Null => "null".to_string(),
            Object::Error(message) => format!("ERROR: {message}"),
            Object::Boolean(bool) => bool.to_string(),
            Object::Integer(int) => int.to_string(),
            Object::ReturnValue(value) => value.to_string(),
            Object::Function(params, body, _) => {
                let params = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({params}) {{\n{body}\n}}")
            }
        }
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(self, Object::Boolean(false) | Object::Null)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Object::Null => "NULL",
            Object::Error(_) => "ERROR",
            Object::Boolean(_) => "BOOLEAN",
            Object::Integer(_) => "INTEGER",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Function(_, _, _) => "FUNCTION",
        };
        write!(f, "{kind}")
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        if value {
            return TRUE;
        }
        FALSE
    }
}

#[derive(Debug)]
pub struct Environment {
    store: HashMap<Identifier, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: Identifier, value: Object) {
        self.store.insert(name, value);
    }
}
