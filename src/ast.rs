use crate::token::Token;

pub type Ast = Vec::<Statement>;

pub enum Statement {
    Let(LetStatement),
    NoStatement,
}

pub struct LetStatement {
    name: Token,
    var_type: Token,
    expression: Expression
}

pub enum Expression {
    Value(Token)
}
