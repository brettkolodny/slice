use crate::token::Token;

pub type Ast = Vec<Statement>;

#[derive(Clone, Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(Expression),
    End,
}

impl Statement {
    pub fn new_let_statement(name: Token, var_type: Token, expression: Expression) -> Self {
        Statement::Let(LetStatement::new(name, var_type, expression))
    }
}

#[derive(Clone, Debug)]
pub struct LetStatement {
    pub name: Token,
    pub var_type: Token,
    pub expression: Expression,
}

impl LetStatement {
    pub fn new(name: Token, var_type: Token, expression: Expression) -> Self {
        LetStatement {
            name,
            var_type,
            expression,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Value(Token),
}
