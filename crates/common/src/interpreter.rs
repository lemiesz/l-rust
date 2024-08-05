use crate::{
    expression::{Expr, ExprKind, Stmt},
    token::{self, Token, TokenType},
    value::{self, Value},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, result};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{message}\n[line {line}]")]
    Runtime { message: String, line: usize },

    #[error("Returning {value:?}")]
    Return { value: Value },
}
#[derive(Clone, Default, Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), Error> {
        let lexeme = name.lexeme();
        self.get(name).map(|_| self.define(lexeme, value))
    }

    pub fn get(&self, token: &Token) -> Result<Value, Error> {
        let lexeme = token.lexeme();
        if let Some(value) = self.values.get(lexeme) {
            return Ok(value.clone());
        } else {
            Err(Error::Runtime {
                message: format!("Undefined varliable {lexeme}"),
                line: token.line,
            })
        }
    }
}

pub struct Interpreter {
    expressions: Vec<Expr>,
    enviorment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            enviorment: Rc::new(RefCell::new(Environment::default())),
            expressions: vec![],
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&self, statments: Vec<Stmt>) {
        for statement in statments {
            if let Err(error) = self.execute(statement) {
                println!("[Error]: {error}");
            }
        }
    }

    pub fn execute(&self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Expression(expression) => {
                self.evaluate(expression)?;
            }
            Stmt::Print(expession) => {
                let value = self.evaluate(expession)?;
                println!("{}", value);
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)?
                } else {
                    Value::Nil
                };

                self.enviorment.borrow_mut().define(name.lexeme(), value);
            }
            Stmt::Block(_) => todo!(),
            Stmt::Class {
                name,
                superclass,
                methods,
            } => todo!(),
            Stmt::Function { name, params, body } => todo!(),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::While { condition, body } => todo!(),
        }
        Ok(())
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value, Error> {
        match expr.kind {
            ExprKind::Literal(value) => Ok(value.unwrap()),
            ExprKind::Assign {
                ref name,
                ref value,
            } => {
                let value = self.evaluate(*value.clone())?;

                let _ = self.enviorment.borrow_mut().assign(name, value.to_owned());
                Ok(value)
            }
            ExprKind::Binary {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate(*left)?;
                let right_result = self.evaluate(*right)?;
                match operator.token_type {
                    TokenType::MINUS => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::SLASH => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::STAR => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::PLUS => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be two numbers or two strings".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::GREATER => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::GreaterEqual => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::LESS => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::LessEqual => match (left_result, right_result) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                        _ => Err(Error::Runtime {
                            message: "Operands must be numbers".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::BangEqual => match (left_result, right_result) {
                        (l, r) => Ok(Value::Boolean(l != r)),
                    },
                    TokenType::EqualEqual => match (left_result, right_result) {
                        (l, r) => Ok(Value::Boolean(l == r)),
                    },
                    _ => unreachable!(),
                }
            }
            ExprKind::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
            ExprKind::Get { object, name } => todo!(),
            ExprKind::Grouping(inner) => self.evaluate(*inner),
            ExprKind::Logical {
                left,
                operator,
                right,
            } => todo!(),
            ExprKind::Set {
                object,
                name,
                value,
            } => todo!(),
            ExprKind::Super { keyword, method } => todo!(),
            ExprKind::This(_) => todo!(),
            ExprKind::Unary { operator, right } => {
                let result = self.evaluate(*right)?;
                match operator.token_type {
                    TokenType::MINUS => match result {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(Error::Runtime {
                            message: "Operand must be a number".to_string(),
                            line: operator.line,
                        }),
                    },
                    TokenType::BANG => match result {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        Value::Nil => Ok(Value::Boolean(true)),
                        _ => Err(Error::Runtime {
                            message: "Operand must be a boolean".to_string(),
                            line: operator.line,
                        }),
                    },
                    _ => unreachable!(),
                }
            }
            ExprKind::Variable(ref name) => self.lookup_variable(name, &expr),
        }
    }

    fn lookup_variable(&self, name: &Token, expr: &Expr) -> Result<Value, Error> {
        self.enviorment.borrow().get(name)
    }
}

// add tests for this module
#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        expression::{Expr, ExprKind},
        interpreter::{Environment, Interpreter},
        token::{Token, TokenType},
        value::Value,
    };

    // Add a test that tests evaluating a literal expression
    #[test]
    fn test_evaluating_literal() {
        let interpreter = Interpreter::new();
        let expr = super::Expr::new(super::ExprKind::Literal(Some(super::Value::Number(5.0))));
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(5.0));
    }

    // Add a test that tests evaluating a grouping expression
    #[test]
    fn test_evaluating_grouping() {
        let interpreter = super::Interpreter::new();
        let expr = super::Expr::new(super::ExprKind::Grouping(Box::new(super::Expr::new(
            super::ExprKind::Literal(Some(super::Value::Number(5.0))),
        ))));
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(5.0));
    }

    /**
     * Add tests for evaluating unary expressions
     * It should test the unary minus (-) operator case
     *
     * Also, test the unary bang (!) operator case. !Nil should be true
     * !false should be true
     * !true should be false
     */
    #[test]
    fn test_evaluating_unary() {
        let interpreter = super::Interpreter::new();
        let expr = super::Expr::new(super::ExprKind::Unary {
            operator: super::Token::new(super::TokenType::MINUS, "-".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(-5.0));

        let expr = super::Expr::new(super::ExprKind::Unary {
            operator: super::Token::new(super::TokenType::BANG, "!".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Nil,
            )))),
        });

        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(true));

        let expr = super::Expr::new(super::ExprKind::Unary {
            operator: super::Token::new(super::TokenType::BANG, "!".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Boolean(false),
            )))),
        });

        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(true));
    }

    /**
     * Add tests for evaluating binary arithmetic expressions
     */
    #[test]
    fn test_evaluating_binary() {
        let interpreter = super::Interpreter::new();
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::MINUS, "-".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(0.0));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::SLASH, "/".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(1.0));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::STAR, "*".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(25.0));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::PLUS, "+".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Number(10.0));
    }

    /**
     * Add tests for evaluating binary comparison expressions
     */
    #[test]
    fn test_evaluating_binary_comparison() {
        let interpreter = super::Interpreter::new();
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::GREATER, ">".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(false));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::GreaterEqual, ">=".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(true));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::LESS, "<".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(false));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::LessEqual, "<=".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(true));

        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
            operator: super::Token::new(super::TokenType::BangEqual, "!=".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                super::Value::Number(5.0),
            )))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(false));

        // test that nil is equal to nil
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(Value::Nil)))),
            operator: super::Token::new(super::TokenType::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(Value::Nil)))),
        });
        let result = interpreter.evaluate(expr).unwrap();
        assert_eq!(result, super::Value::Boolean(true));
    }

    #[test]
    fn test_variable_expression() {
        // Create an interpreter using the default implementation
        let interpreter = Interpreter::default();

        // Define a variable "x" with an initial value 10 in the environment
        let initial_value = Value::Number(10.0);
        let var_name = Token::new(TokenType::VAR, "x".to_string(), Some("x".to_string()), 0);
        interpreter
            .enviorment
            .borrow_mut()
            .define("x", initial_value.clone());

        // Create a variable expression that refers to "x"
        let var_expr = Expr::new(ExprKind::Variable(var_name.clone()));

        // Evaluate the variable expression
        let result = interpreter.evaluate(var_expr).unwrap();

        // Check that the result is equal to the initial value of "x"
        assert_eq!(result, initial_value);
    }

    #[test]
    fn test_var_initialization_statement() {
        // Create an interpreter using the default implementation
        let interpreter = Interpreter::default();

        // Define a token for the variable name
        let var_name = Token::new(TokenType::VAR, "y".to_string(), Some("y".to_string()), 0);

        // Define the initial value for the variable (e.g., 42)
        let initial_value = Value::Number(42.0);

        // Create a variable declaration statement
        let var_stmt = super::Stmt::Var {
            name: var_name.clone(),
            initializer: Some(Expr::new(ExprKind::Literal(Some(initial_value.clone())))),
        };

        // Execute the statement
        interpreter.execute(var_stmt).unwrap();

        // Check that the variable "y" has been correctly initialized in the environment
        let result = interpreter.enviorment.borrow().get(&var_name).unwrap();
        assert_eq!(result, initial_value);
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::default();
        let value1 = Value::String("1234".to_string());

        env.define("z", value1.clone());
        let token = Token::new(TokenType::VAR, "z".to_string(), Some("z".to_string()), 0);

        assert_eq!(env.get(&token).unwrap(), value1);
    }

    #[test]
    fn test_get_nonexistent_variable() {
        let env = Environment::default();

        let token = Token::new(TokenType::VAR, "y".to_string(), Some("y".to_string()), 0);

        let result = env.get(&token);

        assert!(result.is_err());
        if let Err(Error::Runtime { message, line }) = result {
            assert_eq!(line, 0);
        } else {
            panic!("Expected runtime error for undefined variable.");
        }
    }

    #[test]
    fn test_redefine_variable() {
        let mut env = Environment::default();
        let value1 = Value::String("1234".to_string());
        let value2 = Value::String("1".to_string());

        env.define("z", value1);
        env.define("z", value2.clone());

        let token = Token::new(TokenType::VAR, "z".to_string(), Some("z".to_string()), 0);
        assert_eq!(env.get(&token).unwrap(), value2);
    }
}
