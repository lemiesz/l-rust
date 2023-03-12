/**
* Here we implement a parsed based on https://craftinginterpreters.com/parsing-expressions.html
*
* The grammar is:
*  expression     → equality ;
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
    expression::{Expr, ExprKind},
    token::{self, Token, TokenType},
    value::Value,
};
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
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

type ParseResult = Result<Expr, Error>;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_owned(),
            position: RefCell::new(0),
        }
    }

    pub fn parse(&self) -> ParseResult {
        return self.expression();
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
        return None;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn increment_position(&self) {
        *self.position.borrow_mut() += 1;
    }

    fn advance(&self) -> Token {
        if !self.is_at_end() {
            self.increment_position();
        }
        return self.previous();
    }

    fn previous(&self) -> Token {
        let prev_position = *self.position.borrow() - 1;
        // let prev_position = self.position.load(Ordering::Relaxed) - 1;
        return self.tokens[prev_position].clone();
    }

    fn peek(&self) -> Token {
        return self.tokens[*self.position.borrow()].clone();
    }

    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        return Err(self.error(self.peek(), message.to_string()));
    }

    fn error(&self, token: Token, message: String) -> Error {
        let msg = if token.token_type == TokenType::EOF {
            format!("{} at end", message.as_str())
        } else {
            format!(
                "line: {} at {}, {}",
                token.line.to_string(),
                token.to_lexme(),
                message.as_str(),
            )
        };
        return Error::ParseErrorCustom(msg.to_string());
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    // express -> equality
    fn expression(&self) -> ParseResult {
        return self.equality();
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&self) -> ParseResult {
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
        return Ok(expr);
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&self) -> ParseResult {
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
        return Ok(expr);
    }

    // term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&self) -> ParseResult {
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
        return Ok(expr);
    }

    // factor -> unray ((* | /) unray)*
    fn factor(&self) -> ParseResult {
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
        return Ok(expr);
    }

    // unary -> ( "!" | "-" ) unary | primary
    fn unary(&self) -> ParseResult {
        if self
            .match_token(vec![TokenType::BANG, TokenType::MINUS])
            .is_some()
        {
            let operator = self.previous();
            let right = bx![self.unary()?];
            Expr::new(ExprKind::Unary { operator, right });
        }
        return self.primary();
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&self) -> ParseResult {
        let token = self.advance();
        match token.token_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => Ok(Expr::new(ExprKind::Literal(Some(Value::from_token(token))))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return Ok(Expr::new(ExprKind::Grouping(Box::new(expr))));
            }
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
        expression::{Expr, ExprKind},
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
}
