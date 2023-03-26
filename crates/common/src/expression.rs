use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
};

use uuid::Uuid;

use crate::{
    token::{Token, TokenType},
    value::Value,
};

// copying from here https://github.com/mchlrhw/loxide/blob/main/treewalk/src/ast.rs
// also found an interesting implementation using macros here https://github.com/abesto/jlox-rs/blob/main/src/ast.rs

#[derive(Clone, Debug)]
pub enum ExprKind {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping(Box<Expr>),
    Literal(Option<Value>),
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable(Token),
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: Uuid,
    pub kind: ExprKind,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        self.id == other.id
    }
}

impl Eq for Expr {}

fn parenthesize(name: &str, exprs: &[Box<Expr>]) -> String {
    let mut result = String::new();
    result.push('(');
    result.push_str(name);
    for expr in exprs {
        result.push(' ');
        result.push_str(&expr.clone().to_string());
    }
    result.push(')');
    result
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        let id = Uuid::new_v4();
        Self { id, kind }
    }

    pub fn to_string(&self) -> String {
        match self.kind.clone() {
            ExprKind::Binary {
                left,
                operator,
                right,
            } => parenthesize(&operator.to_lexme(), &[left, right]),
            ExprKind::Grouping(expression) => parenthesize("group", &[expression]),
            ExprKind::Literal(literal) => {
                if literal.is_none() {
                    return "nil".to_string();
                }
                let mut result = String::new();
                result.push('(');
                result.push_str(literal.unwrap().to_string().as_str());
                result.push(')');
                result
            }
            ExprKind::Unary { operator, right } => parenthesize(&operator.to_lexme(), &[right]),
            ExprKind::Assign { name, value } => {
                let mut result = String::new();
                result.push('(');
                result.push_str("=");
                result.push(' ');
                result.push_str(&name.to_lexme());
                result.push(' ');
                result.push_str(&value.to_string());
                result.push(')');
                result
            }

            ExprKind::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
            ExprKind::Get { object, name } => todo!(),
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
            ExprKind::Variable(token) => return token.to_lexme(),
        }
    }
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// impl fmt::Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

/**
 * program -> statement* EOF;
 * statement -> exprStmt
 *             | printStmt;
 * exprStmt -> expression ";";
 * printStmt -> "print" expression ";";
 */
#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class {
        name: Token,
        superclass: Option<Expr>,
        methods: Vec<Stmt>,
    },
    Expression(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn prints_basic_ast() {
        let expr = super::Expr::new(super::ExprKind::Literal(Some(Value::Number(1.))));
        assert_eq!(expr.to_string(), "(1)");
    }

    // an AST with a binary expression
    #[test]
    fn prints_ast_with_binary_expression() {
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                Value::Number(1.0),
            )))),
            operator: super::Token::new(super::TokenType::PLUS, "+".to_owned(), Option::None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                Value::Number(2.0),
            )))),
        });
        assert_eq!(expr.to_string(), "(+ (1) (2))");
    }

    /*
     a test of the following java code but using the current implementation

         Expr expression = new Expr.Binary(
       new Expr.Unary(
           new Token(TokenType.MINUS, "-", null, 1),
           new Expr.Literal(123)),
       new Token(TokenType.STAR, "*", null, 1),
       new Expr.Grouping(
           new Expr.Literal(45.67)));
    */
    #[test]
    fn prints_ast_with_nested_binary_expression() {
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Unary {
                operator: super::Token::new(
                    super::TokenType::MINUS,
                    "-".to_owned(),
                    Option::None,
                    1,
                ),
                right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                    Value::Number(123.),
                )))),
            })),
            operator: super::Token::new(super::TokenType::STAR, "*".to_owned(), Option::None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Grouping(Box::new(
                super::Expr::new(super::ExprKind::Literal(Some(Value::Number(45.67)))),
            )))),
        });
        assert_eq!(expr.to_string(), "(* (- (123)) (group (45.67)))");
    }

    /*
       Test of function representing the code 25 + 10 * (2/4)
    */
    #[test]
    fn prints_ast_with_nested_binary_expression_2() {
        let expr = super::Expr::new(super::ExprKind::Binary {
            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                Value::Number(25.),
            )))),
            operator: super::Token::new(super::TokenType::PLUS, "+".to_owned(), Option::None, 1),
            right: Box::new(super::Expr::new(super::ExprKind::Binary {
                left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                    Value::Number(10.),
                )))),
                operator: super::Token::new(
                    super::TokenType::STAR,
                    "*".to_owned(),
                    Option::None,
                    1,
                ),
                right: Box::new(super::Expr::new(super::ExprKind::Grouping(Box::new(
                    super::Expr::new(super::ExprKind::Binary {
                        left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                            Value::Number(2.),
                        )))),
                        operator: super::Token::new(
                            super::TokenType::SLASH,
                            "/".to_owned(),
                            Option::None,
                            1,
                        ),
                        right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                            Value::Number(4.),
                        )))),
                    }),
                )))),
            })),
        });
        assert_eq!(expr.to_string(), "(+ (25) (* (10) (group (/ (2) (4)))))");
    }

    /*
      test the express the assignment of a variable
      let a = 1;
    */
    #[test]
    fn prints_ast_with_assignment() {
        let expr = super::Expr::new(super::ExprKind::Assign {
            name: super::Token::new(
                super::TokenType::IDENTIFIER,
                "a".to_owned(),
                Option::None,
                1,
            ),
            value: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                Value::Number(1.),
            )))),
        });
        assert_eq!(expr.to_string(), "(= a (1))");
    }

    /*
      test that the result of the expression in test prints_ast_with_nested_binary_expression_2 gets assigned to a variable a
    */
    #[test]
    fn prints_ast_with_assignment_and_expression() {
        let expr = super::Expr::new(super::ExprKind::Assign {
            name: super::Token::new(
                super::TokenType::IDENTIFIER,
                "a".to_owned(),
                Option::None,
                1,
            ),
            value: Box::new(super::Expr::new(super::ExprKind::Binary {
                left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                    Value::Number(25.),
                )))),
                operator: super::Token::new(
                    super::TokenType::PLUS,
                    "+".to_owned(),
                    Option::None,
                    1,
                ),
                right: Box::new(super::Expr::new(super::ExprKind::Binary {
                    left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                        Value::Number(10.),
                    )))),
                    operator: super::Token::new(
                        super::TokenType::STAR,
                        "*".to_owned(),
                        Option::None,
                        1,
                    ),
                    right: Box::new(super::Expr::new(super::ExprKind::Grouping(Box::new(
                        super::Expr::new(super::ExprKind::Binary {
                            left: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                                Value::Number(2.),
                            )))),
                            operator: super::Token::new(
                                super::TokenType::SLASH,
                                "/".to_owned(),
                                Option::None,
                                1,
                            ),
                            right: Box::new(super::Expr::new(super::ExprKind::Literal(Some(
                                Value::Number(4.),
                            )))),
                        }),
                    )))),
                })),
            })),
        });
        assert_eq!(
            expr.to_string(),
            "(= a (+ (25) (* (10) (group (/ (2) (4))))))"
        );
    }
}
