pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

/**
 *
 * package com.craftinginterpreters.lox;

class Token {
  final TokenType type;
  final String lexeme;
  final Object literal;
  final int line;

  Token(TokenType type, String lexeme, Object literal, int line) {
    this.type = type;
    this.lexeme = lexeme;
    this.literal = literal;
    this.line = line;
  }

  public String toString() {
    return type + " " + lexeme + " " + literal;
  }
}

 * */

pub struct Token<'code> {
    token_type: TokenType,
    lexeme: &'code str,
    literal: Option<String>,
    line: usize,
}

impl<'code> Token<'code> {
    pub fn new(
        t_type: TokenType,
        lexeme: &'code str,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Token {
            token_type: t_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }

    pub fn toString(self) {
        return self.token_type + " " + self.lexeme + " " + self.literal;
    }
}
