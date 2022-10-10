use std::{collections::HashMap, str::FromStr};

use super::token::{self, Token, TokenType};

pub struct Scanner<'code> {
    code: &'code String,
    tokens: Vec<Token<'code>>,
    start: usize,
    current: usize,
    line: usize,
    two_char_tokens: HashMap<char, char>,
    had_error: bool,
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
            had_error: false,
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
                    None => match token_type {
                        TokenType::SPACE | TokenType::SLASHRETURN | TokenType::TAB => {}
                        TokenType::NEWLINE => {
                            self.line = self.line + 1;
                            return;
                        }
                        TokenType::SLASH => {
                            while self.peek().unwrap() != '\n' && !self.is_at_end() {
                                self.advance();
                            }
                        }
                        TokenType::QUOTESTRING => {
                            self.string();
                        }
                        _ => {
                            if self.is_digit(c) {
                                self.number()
                            }
                            self.add_token(token_type)
                        }
                    },
                }
            }
            Err(_) => {
                self.had_error = true;
                println!("Error could not parse token {}, line {}", c, self.line)
            }
        };
    }

    fn number(&mut self) {
        while self.is_digit(self.peek().unwrap()) {
            self.advance();
        }

        if self.peek().unwrap() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek().unwrap()) {
                self.advance();
            }
        }

        let result = self.code.get(self.start..self.current);
        match result {
            Some(literal) => {
                self.add_token_with_literal(TokenType::NUMBER, Some(literal.to_string()));
            }
            None => {
                println!("Could not parse literal");
            }
        }
    }

    fn string(&mut self) {
        while self.peek().unwrap() != '"' && !self.is_at_end() {
            if self.peek().unwrap() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Error found unterminated string on line {}", self.line);
        }
        self.advance();

        let result = self.code.get(self.start + 1..self.current - 1);
        match result {
            Some(literal) => {
                self.add_token_with_literal(TokenType::STRING, Some(literal.to_string()));
            }
            None => {
                println!("Could not parse literal");
            }
        }
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    // peeks to see what the next character is
    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        return self.char_at(self.current);
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.code.len() {
            return '\0';
        }
        return self.char_at(self.current + 1).unwrap();
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

    fn char_at(&mut self, n: usize) -> Option<char> {
        return self.code.chars().nth(n);
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
