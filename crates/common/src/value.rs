use std::fmt;

use crate::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(s), Value::Boolean(o)) => s == o,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(s), Value::Number(o)) => s == o,
            (Value::String(s), Value::String(o)) => s == o,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            // Self::Callable(c) => write!(f, "{c}"),
            // Self::Instance(i) => write!(f, "{}", i.borrow()),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

impl Value {
    pub fn from_token(token: Token) -> Value {
        match token.token_type {
            TokenType::FALSE => Value::Boolean(false),
            TokenType::TRUE => Value::Boolean(true),
            TokenType::NIL => Value::Nil,
            TokenType::NUMBER => {
                Value::Number(token.literal.clone().unwrap().parse::<f64>().unwrap())
            }
            TokenType::STRING => Value::String(token.literal.clone().unwrap()),
            _ => panic!("Not a supported value token"),
        }
    }
}
