use std::fmt::{Debug, Display, Formatter};

use strum_macros::EnumString;

#[derive(EnumString, Debug)]
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
    SEMICOLON,
    #[strum(serialize = "/")]
    SLASH,
    #[strum(serialize = "*")]
    STAR,

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

    #[strum(serialize = "EOF")]
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        Debug::fmt(self, f)
    }
}

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

    pub fn toString(self) -> String {
        let token_as_string = self.token_type.to_string();
        return token_as_string + " " + self.lexeme + " " + self.literal.unwrap().as_str();
    }
}
