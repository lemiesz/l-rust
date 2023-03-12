use crate::{
    expression::{Expr, ExprKind},
    token::{Token, TokenType},
    value::{self, Value},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{message}\n[line {line}]")]
    Runtime { message: String, line: usize },

    #[error("Returning {value:?}")]
    Return { value: Value },
}

struct Interpreter {
    expressions: Vec<Expr>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            expressions: vec![],
        }
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value, Error> {
        match expr.kind {
            ExprKind::Literal(value) => Ok(value.unwrap()),
            ExprKind::Assign { name, value } => todo!(),
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
            ExprKind::Variable(_) => todo!(),
        }
    }
}

// add tests for this module
#[cfg(test)]
mod test {
    use crate::value::Value;

    // Add a test that tests evaluating a literal expression
    #[test]
    fn test_evaluating_literal() {
        let interpreter = super::Interpreter::new();
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
}
