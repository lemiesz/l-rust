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
        let c = self.advance();
        match c {
            ')' => self.add_token(TokenType::RightParen),
            '(' => self.add_token(TokenType::LeftParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            _ => println!("Error could not parse token {}, line {}", c, self.line),
        }
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
