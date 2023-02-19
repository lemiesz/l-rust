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
};
use thiserror::Error;

macro_rules! bx {
    ($e: expr) => {
        Box::new($e)
    };
}

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("parse error")]
    ParseError,
}

type ParseResult = Result<Expr, Error>;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_owned(),
            position: 0,
        }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return Some(self.advance());
            }
        }
        return None;
    }

    fn check(&self, token_type: TokenType) -> bool {
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.position - 1].clone();
    }

    fn peek(&self) -> Token {
        return self.tokens[self.position].clone();
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(Error::ParseError)
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    // express -> equality
    fn expression(&mut self) -> ParseResult {
        return self.equality();
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ParseResult {
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
    fn comparison(&mut self) -> ParseResult {
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
    fn term(&mut self) -> ParseResult {
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
    fn factor(&mut self) -> ParseResult {
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
    fn unary(&mut self) -> ParseResult {
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
    fn primary(&mut self) -> ParseResult {
        let token = self.advance();
        match token.token_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => Ok(Expr::new(ExprKind::Literal(token.literal.clone()))),
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return expr;
            }
            _ => Err(Error::ParseError),
        }
    }
}
