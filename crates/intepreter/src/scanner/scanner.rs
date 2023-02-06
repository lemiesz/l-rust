use common::token::{Token, TokenType};
use std::{collections::HashMap, str::FromStr};

pub struct Scanner {
    code: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

/**
 * Basic scanner implementation
 **/
impl Scanner {
    pub fn new(code: String) -> Self {
        Scanner {
            code,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            had_error: false,
        }
    }

    pub fn debug_print(self) {
        println!("Tokens:");
        for token in &self.tokens {
            println!("{:?}", token);
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
        return self.current > self.code.len() - 1;
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        if c.is_alphabetic() {
            return self.identifier();
        }
        // Because of the nasty matcher that needs to be refactored.
        // We treat the case of a literal digit seperatly. Since it cant be pattern matched
        // as no tokens exist for it.
        if c.is_digit(10) {
            return self.number();
        }
        // This code is particularly nasty
        // Not sure if it stems from my misunderstanding of rust, or if im just overengineering
        // Basically what I try to do is check to see if a character is a two-character token,
        // If it is then I add the two char token, otherwise I build a two char token and add that
        match TokenType::from_str(c.to_string().as_str()) {
            Ok(token_type) => {
                let t = TWO_CHAR_TOKENS.get(&c);
                let token_to_add: TokenType;

                match t {
                    Some(item) => {
                        let second_char = item.clone();
                        // concantate double char token into 1 string then create token
                        let double_token_str = format!("{}{}", c, second_char);
                        token_to_add = self.match_double(
                            *item,
                            token_type,
                            TokenType::from_str(&double_token_str).unwrap(),
                        );
                        self.add_token(token_to_add)
                    }
                    None => match token_type {
                        TokenType::SPACE | TokenType::SLASHRETURN | TokenType::TAB => {}
                        TokenType::SEMICOLON => {
                            if self.peek_next() == '\n' {
                                self.advance();
                            } else {
                                self.line = self.line + 1;
                            }
                        }
                        TokenType::NEWLINE => {
                            self.line = self.line + 1;
                            return;
                        }
                        TokenType::SLASH => {
                            if self.match_token_and_advance('/') {
                                while self.peek() != '\n' && !self.is_at_end() {
                                    self.advance();
                                }
                            } else {
                                self.add_token(token_type);
                            }
                        }
                        TokenType::QUOTESTRING => {
                            self.string();
                        }
                        _ => self.add_token(token_type),
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
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
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
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
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

    // peeks to see what the next character is
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.char_at(self.current);
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.code.len() {
            return '\0';
        }
        return self.char_at(self.current + 1);
    }

    fn match_token_and_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.char_at(self.current) != expected {
            return false;
        }
        self.current = self.current + 1;
        return true;
    }

    // Checks to see if the current token is a special character
    // if so we have scanned a 2 char token return that, if not return the 1 char token
    // if at end of file return EOF
    fn match_double(
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
                return '\0';
            }
        }
    }

    fn char_at(&mut self, n: usize) -> char {
        match self.code.chars().nth(n) {
            Some(c) => return c,
            None => panic!("Could not read {} th char", n.to_string()),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Option::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        match self.code.get(self.start..self.current) {
            Some(lexeme) => {
                let token = Token::new(token_type, String::from(lexeme), literal, self.line);
                self.tokens.push(token);
            }
            None => self.tokens.push(Token::new(
                TokenType::EOF,
                String::from("\0"),
                literal,
                self.line,
            )),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.code.get(self.start..self.current).unwrap();
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::IDENTIFIER);
        self.add_token(*token_type);
    }
}

// declare a hashmap of identifiers to token type
lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::AND);
        m.insert("class", TokenType::CLASS);
        m.insert("else", TokenType::ELSE);
        m.insert("false", TokenType::FALSE);
        m.insert("for", TokenType::FOR);
        m.insert("fun", TokenType::FUN);
        m.insert("if", TokenType::IF);
        m.insert("nil", TokenType::NIL);
        m.insert("or", TokenType::OR);
        m.insert("print", TokenType::PRINT);
        m.insert("return", TokenType::RETURN);
        m.insert("super", TokenType::SUPER);
        m.insert("this", TokenType::THIS);
        m.insert("true", TokenType::TRUE);
        m.insert("var", TokenType::VAR);
        m.insert("while", TokenType::WHILE);
        m
    };
}

lazy_static! {
    static ref TWO_CHAR_TOKENS: HashMap<char, char> =
        HashMap::from([('!', '='), ('=', '='), ('<', '='), ('>', '=')]);
}
