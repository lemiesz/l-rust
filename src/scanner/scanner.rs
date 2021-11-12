use std::{collections::HashMap, str::FromStr};

use super::token::{Token, TokenType};

pub struct Scanner<'code> {
    code: &'code String,
    tokens: Vec<Token<'code>>,
    start: usize,
    current: usize,
    line: usize,
    two_char_tokens: HashMap<char, char>,
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
            two_char_tokens: HashMap::from([('!', '='), ('=', '='), ('<', '='), ('>', '=')]),
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        println!("{}", self.code);
    }

    pub fn is_at_end(&mut self) -> bool {
        return self.current > self.code.len().try_into().unwrap();
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        // This code is particularly nasty
        // Not sure if it stems from my misunderstanding of rust, or if im just overengineering
        // Basically what I try to do is check to see if a character is a two-character token,
        // If it is then I add the two char token, otherwise I build a two char token and add that
        match TokenType::from_str(c.to_string().as_str()) {
            Ok(token_type) => {
                let t = self.two_char_tokens.get(&c);
                let token_to_add: TokenType;
                match t {
                    Some(item) => {
                        let second_char = item.clone();
                        // concantate double char token into 1 string then create token
                        let double_token_str = format!("{}{}", c, second_char);
                        token_to_add = self.match_char(
                            *item,
                            token_type,
                            TokenType::from_str(&double_token_str).unwrap(),
                        );
                        self.add_token(token_to_add)
                    }
                    None => self.add_token(token_type),
                }
            }
            Err(_) => println!("Error could not parse token {}, line {}", c, self.line),
        };
    }

    // Checks to see if the current token is a special character
    // if so we have scanned a 2 char token return that, if not return the 1 char token
    // if at end of file return EOF
    fn match_char(
        &mut self,
        expected: char,
        one_char_token: TokenType,
        two_char_token: TokenType,
    ) -> TokenType {
        if self.is_at_end() {
            return TokenType::EOF;
        }
        if self.code.chars().nth(self.current).unwrap() != expected {
            return one_char_token;
        };
        self.current += 1;
        return two_char_token;
    }

    fn advance(&mut self) -> char {
        match self.code.chars().nth(self.current) {
            Some(c) => {
                self.current += 1;
                return c;
            }
            None => {
                self.current += 1;
                return '\0';
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Option::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        match self.code.get(self.start..self.current) {
            Some(lexeme) => {
                let token = Token::new(token_type, lexeme, literal, self.line);
                self.tokens.push(token);
            }
            None => self
                .tokens
                .push(Token::new(TokenType::EOF, "\0", literal, self.line)),
        }
    }
}
