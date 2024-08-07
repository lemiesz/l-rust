use std::fmt::{Debug, Display, Formatter};

use strum_macros::EnumString;

#[derive(EnumString, Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,
    #[strum(serialize = ",")]
    COMMA,
    #[strum(serialize = ".")]
    DOT,
    #[strum(serialize = "-")]
    MINUS,
    #[strum(serialize = "+")]
    PLUS,
    #[strum(serialize = ":")]
    COLON,
    #[strum(serialize = "/")]
    SLASH,
    #[strum(serialize = "*")]
    STAR,
    #[strum(serialize = "\"")]
    QUOTESTRING,

    // One or two character tokens.
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "!=")]
    BangEqual,
    #[strum(serialize = "=")]
    EQUAL,
    #[strum(serialize = "==")]
    EqualEqual,
    #[strum(serialize = ">")]
    GREATER,
    #[strum(serialize = ">=")]
    GreaterEqual,
    #[strum(serialize = "<")]
    LESS,
    #[strum(serialize = "<=")]
    LessEqual,

    // Literals.
    #[strum(serialize = "")]
    IDENTIFIER,
    #[strum(serialize = "String")]
    STRING,
    #[strum(serialize = "Number")]
    NUMBER,

    // whitespace
    #[strum(serialize = " ")]
    SPACE,
    #[strum(serialize = "\r")]
    SLASHRETURN,
    #[strum(serialize = "\t")]
    TAB,
    #[strum(serialize = "\n")]
    NEWLINE,
    #[strum(serialize = ";")]
    SEMICOLON,

    // Keywords.
    #[strum(serialize = "and")]
    AND,
    #[strum(serialize = "class")]
    CLASS,
    #[strum(serialize = "else")]
    ELSE,
    #[strum(serialize = "false")]
    FALSE,
    #[strum(serialize = "fun")]
    FUN,
    #[strum(serialize = "for")]
    FOR,
    #[strum(serialize = "if")]
    IF,
    #[strum(serialize = "nil")]
    NIL,
    #[strum(serialize = "or")]
    OR,
    #[strum(serialize = "print")]
    PRINT,
    #[strum(serialize = "return")]
    RETURN,
    #[strum(serialize = "super")]
    SUPER,
    #[strum(serialize = "this")]
    THIS,
    #[strum(serialize = "true")]
    TRUE,
    #[strum(serialize = "var")]
    VAR,
    #[strum(serialize = "while")]
    WHILE,

    #[strum(serialize = "\0")]
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type: t_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(self) -> String {
        let token_as_string = self.token_type.to_string();
        return token_as_string;
    }

    pub fn to_lexme(self) -> String {
        return self.lexeme;
    }

    pub fn typ(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
