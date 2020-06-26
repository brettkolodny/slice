use crate::token::Token;

pub type Ast = Vec<Statement>;

<<<<<<< HEAD
#[derive(Clone, Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(Expression),
    End,
=======
#[derive(Clone)]
pub enum Statement {
    Let(LetStatement),
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
    NoStatement,
}

impl Statement {
    pub fn new_let_statement(name: Token, var_type: Token, expression: Expression) -> Self {
        Statement::Let(LetStatement::new(name, var_type, expression))
    }
}

<<<<<<< HEAD
#[derive(Clone, Debug)]
=======
#[derive(Clone)]
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
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

<<<<<<< HEAD
#[derive(Clone, Debug)]
=======
#[derive(Clone)]
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
pub enum Expression {
    Value(Token),
}
