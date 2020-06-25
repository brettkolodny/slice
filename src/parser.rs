use std::mem;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

use crate::ast::{Ast, Statement};
use crate::ast;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next();
        let peek_token = lexer.next();

        Parser { lexer, current_token, peek_token }
    }

    pub fn parse_program(&mut self) {
        let mut ast = Ast::new();
        while self.current_token.token_type != TokenType::EOF {
            if self.current_token.token_type == TokenType::Let {
                ast.push(self.pase_let())
            }

            self.advance_tokens();
        }
    }

    pub fn advance_tokens(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.current_token);
        self.peek_token = self.lexer.next();
    }

    pub fn pase_let(&mut self) -> Statement {
        Statement::NoStatement
    }
}
