use std::str::FromStr;

use crate::scanner::token;

use super::token::{Token, TokenType};

pub struct Scanner<'code> {
    code: &'code String,
    tokens: Vec<Token<'code>>,
    start: usize,
    current: usize,
    line: usize,
}

/**
 * Basic scanner implementation
 **/
impl<'code> Scanner<'code> {
    pub fn new(code: &'code String) -> Self {
        Scanner {
            code,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        println!("{}", self.code);
    }

    pub fn is_at_end(&mut self) -> bool {
        return self.current > self.code.len().try_into().unwrap();
    }

    pub fn scan_token(&mut self) {
        let c = self.advance().to_string();
        match TokenType::from_str(c.as_str()) {
            Ok(token_type) => self.add_token(token_type),
            Err(_) => println!("Error could not parse token {}, line {}", c, self.line),
        };
    }

    fn advance(&mut self) -> char {
        let c = self.code.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Option::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = &self.code[self.start..self.current];
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }
}
