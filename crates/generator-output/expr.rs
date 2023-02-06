use super::token::Token;
pub struct Expr {
    pub binary: Expr left, Token operator, Expr right,
    pub grouping: Expr expression,
    pub literal: Object value,
    pub unary: Token operator, Expr right,
}
impl Expr {
}
