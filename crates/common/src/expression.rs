use super::token::Token;

pub struct Expresion<'a> {
    pub left: Box<Expresion<'a>>,
    pub right: Box<Expresion<'a>>,
    pub operator: Token<'a>,
}

impl<'a> Expresion<'a> {
    pub fn new(
        left: Box<Expresion<'a>>,
        right: Box<Expresion<'a>>,
        operator: Token<'a>,
    ) -> Expresion<'a> {
        Expresion {
            left,
            right,
            operator,
        }
    }
}
