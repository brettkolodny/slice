use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::mem;

use crate::ast::{Ast, Expression, Statement};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next();
        let peek_token = lexer.next();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new();
        while self.current_token.token_type != TokenType::EOF {
            if self.current_token.token_type == TokenType::Let {
                let let_statement = self.parse_let()?;
                ast.push(let_statement)
            }

            self.advance_tokens();
        }

        Ok(ast)
    }

    pub fn advance_tokens(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.current_token);
        self.peek_token = self.lexer.next();
    }

    pub fn parse_let(&mut self) -> Result<Statement, String> {
        self.advance_tokens();

        let identifier = self.parse_identifier()?;

        self.advance_tokens();

        if let TokenType::Colon = self.current_token.token_type {
            self.advance_tokens();
        } else {
            return Err(format!(
                "Unexpected token expected ':' {} {}.{}",
                self.current_token, self.current_token.row, self.current_token.col
            ));
        }

        let let_type = self.parse_type()?;

        self.advance_tokens();

        if let TokenType::Assign = self.current_token.token_type {
            self.advance_tokens()
        } else {
            return Err(format!(
                "Unexpected token expected '=' {} {}.{}",
                self.current_token, self.current_token.row, self.current_token.col
            ));
        }

        let expr = self.parse_expression()?;

        self.advance_tokens();

        Ok(Statement::new_let_statement(identifier, let_type, expr))
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        match &self.current_token.token_type {
            TokenType::Str(_)
            | TokenType::Int(_)
            | TokenType::Character(_)
            | TokenType::False
            | TokenType::True => Ok(Expression::Value(
                mem::take(&mut self.current_token))),
            _ => Err(format!(
                "Unexpected token expected value {} {}.{}",
                self.current_token, self.current_token.row, self.current_token.col
            )),
        }
    }

    pub fn parse_type(&mut self) -> Result<Token, String> {
        match &self.current_token.token_type {
            TokenType::StringType
            | TokenType::IntType
            | TokenType::CharType
            | TokenType::BoolType => Ok(mem::take(&mut self.current_token)),
            _ => Err(format!(
                "Unexpected token expected type {} {}.{}",
                self.current_token, self.current_token.row, self.current_token.col
            )),
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Token, String> {
        if let TokenType::Identity(_) = &self.current_token.token_type {
            Ok(mem::take(&mut self.current_token))
        } else {
            Err(format!(
                "Invalid indtifier {}, expected Identity {}.{}",
                self.current_token, self.current_token.row, self.current_token.col
            ))
        }
    }
}
