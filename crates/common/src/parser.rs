/**
* Here we implement a parsed based on https://craftinginterpreters.com/parsing-expressions.html
*
* The grammar is:
*  expression     → assignment ;
   assignment     → IDENTIFIER "=" assignment
                    | equality ;
   equality       → comparison ( ( "!=" | "==" ) comparison )* ;
   comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
   term           → factor ( ( "-" | "+" ) factor )* ;
   factor         → unary ( ( "/" | "*" ) unary )* ;
   unary          → ( "!" | "-" ) unary
                    |primary ;
   primary        → NUMBER | STRING | "true" | "false" | "nil"
                   |"(" expression ")" ;

   heavily inspired by https://github.com/mchlrhw/loxide/blob/main/treewalk/src/parser.rs
*/
use crate::{
    expression::{Expr, ExprKind, Stmt},
    token::{Token, TokenType},
    value::Value,
};
use std::cell::RefCell;
use thiserror::Error;

macro_rules! bx {
    ($e: expr) => {
        Box::new($e)
    };
}

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("parse error: {}", .0)]
    ParseErrorCustom(String),
    #[error("Error on line: {} token: {} parse error: {message}", .token.line, token.token_type.to_string())]
    ParseErrorToken { token: Token, message: String },
    #[error("parse error")]
    ParseErrorGeneric,
}

type ParseResult = Result<Vec<Stmt>, Error>;
type ExprResult = Result<Expr, Error>;
type StmtResult = Result<Stmt, Error>;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: RefCell<usize>,
    pub errors: RefCell<Vec<Error>>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_owned(),
            position: RefCell::new(0),
            errors: RefCell::new(vec![]),
        }
    }

    pub fn parse(&self) -> ParseResult {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt)
            }
        }

        if self.errors.borrow().is_empty() {
            Ok(statements)
        } else {
            Err(self.errors.borrow()[0].clone())
        }
    }

    fn declaration(&self) -> Option<Stmt> {
        let res: StmtResult = if self.match_token(vec![TokenType::VAR]).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        };

        match res {
            Ok(stmt) => Some(stmt),
            Err(err) => {
                self.errors.borrow_mut().push(err);
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&self) -> StmtResult {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;

        let mut initializer = None;
        if self.match_token(vec![TokenType::EQUAL]).is_some() {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable decleration",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn synchronize(&self) {
        self.advance();

        while !self.is_at_end() {
            // if the previous token was a semicolon, we are at the end of a statment
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                // if the next token is one of these, we are at the start of a new statement
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn match_token(&self, token_types: Vec<TokenType>) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return Some(self.advance());
            }
        }
        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn increment_position(&self) {
        *self.position.borrow_mut() += 1;
    }

    fn advance(&self) -> Token {
        if !self.is_at_end() {
            self.increment_position();
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        let prev_position = *self.position.borrow() - 1;
        // let prev_position = self.position.load(Ordering::Relaxed) - 1;
        self.tokens[prev_position].clone()
    }

    fn peek(&self) -> Token {
        return self.tokens[*self.position.borrow()].clone();
    }

    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message.to_string()))
    }

    fn error(&self, token: Token, message: String) -> Error {
        let msg = if token.token_type == TokenType::EOF {
            format!("{} at end", message.as_str())
        } else {
            format!(
                "line: {} at {}, {}",
                token.line,
                token.clone().to_lexme(),
                message.as_str(),
            )
        };
        Error::ParseErrorCustom(msg)
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn statement(&self) -> StmtResult {
        if self.match_token(vec![TokenType::PRINT]).is_some() {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&self) -> StmtResult {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Expression(expr))
    }

    // express -> equality
    fn expression(&self) -> ExprResult {
        self.assignment()
    }

    fn assignment(&self) -> ExprResult {
        let expr = self.equality()?;

        if self.match_token(vec![TokenType::EQUAL]).is_some() {
            let equals = self.previous();
            let value = Box::new(self.assignment()?);

            if let ExprKind::Variable(name) = expr.kind {
                return Ok(Expr::new(ExprKind::Assign {
                    name: name,
                    value: value,
                }));
            }
            return Err(self.error(equals, "Invalid Assignment Target.".to_owned()));
        }
        Ok(expr)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&self) -> ExprResult {
        let mut expr = self.comparison()?;
        while self
            .match_token(vec![TokenType::BangEqual, TokenType::EqualEqual])
            .is_some()
        {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new(ExprKind::Binary {
                left: bx![expr],
                operator,
                right: bx![right],
            });
        }
        Ok(expr)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&self) -> ExprResult {
        let mut expr = self.term()?;
        while self
            .match_token(vec![
                TokenType::GREATER,
                TokenType::GreaterEqual,
                TokenType::LESS,
                TokenType::LessEqual,
            ])
            .is_some()
        {
            let operator = self.previous();
            let rightt = self.term()?;
            expr = Expr::new(ExprKind::Binary {
                left: bx![expr],
                operator,
                right: bx![rightt],
            });
        }
        Ok(expr)
    }

    // term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&self) -> ExprResult {
        let mut expr = self.factor()?;
        while self
            .match_token(vec![TokenType::MINUS, TokenType::PLUS])
            .is_some()
        {
            let operator = self.previous();
            let right = bx![self.factor()?];
            expr = Expr::new(ExprKind::Binary {
                left: bx![expr],
                operator,
                right,
            })
        }
        Ok(expr)
    }

    // factor -> unray ((* | /) unray)*
    fn factor(&self) -> ExprResult {
        let mut expr = self.unary()?;
        while self
            .match_token(vec![TokenType::STAR, TokenType::SLASH])
            .is_some()
        {
            let operator = self.previous();
            let right = bx![self.unary()?];
            expr = Expr::new(ExprKind::Binary {
                left: bx![expr],
                operator,
                right,
            })
        }
        Ok(expr)
    }

    // unary -> ( "!" | "-" ) unary | primary
    fn unary(&self) -> ExprResult {
        if self
            .match_token(vec![TokenType::BANG, TokenType::MINUS])
            .is_some()
        {
            let operator = self.previous();
            let right = bx![self.unary()?];
            Expr::new(ExprKind::Unary { operator, right });
        }
        self.primary()
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&self) -> ExprResult {
        let token = self.advance();
        match token.token_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => Ok(Expr::new(ExprKind::Literal(Some(Value::from_token(token))))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::new(ExprKind::Grouping(Box::new(expr))))
            }
            TokenType::IDENTIFIER => Ok(Expr::new(ExprKind::Variable(self.previous()))),
            _ => Err(Error::ParseErrorToken {
                message: "Did not find a matching primary token".to_string(),
                token: self.peek(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::{Expr, ExprKind, Stmt},
        parser::Parser,
        scanner,
        token::Token,
        token::TokenType,
    };

    /**
     * Test that takes the expression let i = 0; and parses it into an AST
     */
    #[test]
    fn parses_the_result_of_variable_assignment() {
        let mut scanner = scanner::Scanner::new("2 + 2".to_string());

        scanner.scan_tokens();

        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(+ (2) (2))");
    }

    /**
     * Parsing just true/false returns "true" or "false
     */
    #[test]
    fn parses_true_false() {
        let mut scanner = scanner::Scanner::new("true".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(true)");

        let mut scanner = scanner::Scanner::new("false".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(false)");
    }

    /**
     * Order of operations is maintained for multiplcation and division
     */
    #[test]
    fn parses_order_of_operations() {
        let mut scanner = scanner::Scanner::new("2 + 2 * 2".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(+ (2) (* (2) (2)))");

        // and division
        let mut scanner = scanner::Scanner::new("2 + 2 / 2".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(+ (2) (/ (2) (2)))");
    }

    /**
     * can handle expressions such as 1 + 2 * 3 + 4 / 5
     */
    #[test]
    fn parses_complex_expressions() {
        let mut scanner = scanner::Scanner::new("1 + 2 * 3 + 4 / 5".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(expr.to_string(), "(+ (+ (1) (* (2) (3))) (/ (4) (5)))");
    }

    /**
     * Handles expressions that have paranthese and maintain that order of operations
     */
    #[test]
    fn parses_parantheses() {
        let mut scanner = scanner::Scanner::new("(1 + 2) * 3 + 4 / 5".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr.to_string(),
            "(+ (* (group (+ (1) (2))) (3)) (/ (4) (5)))"
        );
    }

    /**
     * Handles a complex expression with parantheses, order of operation, and equality check
     */
    #[test]
    fn parses_complex_parantheses() {
        let mut scanner = scanner::Scanner::new("(1 + 2) * 3 + 4 / 5 == 1".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr.to_string(),
            "(== (+ (* (group (+ (1) (2))) (3)) (/ (4) (5))) (1))"
        );
    }

    /**
     * Handles an even more complex expression with parantheses, order of operation, and multiple quality checks
     */
    #[test]
    fn parses_complex_parantheses_2() {
        let mut scanner = scanner::Scanner::new("((1 + 2) * 3 + 4 / 5 == 1) == 1".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr.to_string(),
            "(== (group (== (+ (* (group (+ (1) (2))) (3)) (/ (4) (5))) (1))) (1))"
        );
    }

    /**
     *  Handles parsing of a full statment ending in a semicolon
     */
    #[test]
    fn parses_full_statement() {
        let mut scanner = scanner::Scanner::new("print 1 + 1;".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 1);
        match stmts.get(0).unwrap() {
            Stmt::Print(expr) => assert_eq!(expr.to_string(), "(+ (1) (1))"),
            _ => panic!("Expected a print statement"),
        }
    }

    /**
     *  Handles pasing of multiple statments
     */
    #[test]
    fn parses_multiple_statments() {
        let mut scanner = scanner::Scanner::new("print 1 + 1; 1 + 2;".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 2);
        match stmts.get(0).unwrap() {
            Stmt::Print(expr) => assert_eq!(expr.to_string(), "(+ (1) (1))"),
            _ => panic!("Expected a print statement"),
        }

        match stmts.get(1).unwrap() {
            Stmt::Expression(expr) => assert_eq!(expr.to_string(), "(+ (1) (2))"),
            _ => panic!("Expected an expression statement"),
        }
    }

    #[test]
    fn prases_var_statement() {
        let mut scanner = scanner::Scanner::new("var i = 1;".to_string());
        scanner.scan_tokens();
        let parser = Parser::new(&scanner.tokens);
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts.len(), 1);
        match stmts.get(0).unwrap() {
            Stmt::Var { initializer, name } => {
                assert_eq!(name.clone().literal.unwrap(), "i".to_string());
                assert!(initializer.is_some());
                // TODO: Not sure how to validate the initialize here
                // assert_eq!(initializer.unwrap().kind, "1".to_string())
            }
            _ => panic!("Expected a variable assignment"),
        }
    }
}
