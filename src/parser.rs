<<<<<<< HEAD
use crate::ast::{Ast, Expression, Statement};
=======
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::mem;

<<<<<<< HEAD
=======
use crate::ast::{Ast, Expression, Statement};

>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
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

<<<<<<< HEAD
    fn advance_tokens(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.current_token);
        self.peek_token = self.lexer.next();
    }

    pub fn parse_program(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new();
        while self.current_token.token_type != TokenType::EOF {
            ast.push(self.parse_statement()?);
=======
    pub fn parse_program(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new();
        while self.current_token.token_type != TokenType::EOF {
            if self.current_token.token_type == TokenType::Let {
                let let_statement = self.parse_let()?;
                ast.push(let_statement)
            }

>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
            self.advance_tokens();
        }

        Ok(ast)
    }

<<<<<<< HEAD
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let(),
            TokenType::Return => self.parse_return(),
            TokenType::End => self.parse_end(),
            _ => Err(format!("Error")),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, String> {
=======
    pub fn advance_tokens(&mut self) {
        std::mem::swap(&mut self.peek_token, &mut self.current_token);
        self.peek_token = self.lexer.next();
    }

    pub fn parse_let(&mut self) -> Result<Statement, String> {
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
        self.advance_tokens();

        let identifier = self.parse_identifier()?;

        self.advance_tokens();

        if let TokenType::Colon = self.current_token.token_type {
            self.advance_tokens();
        } else {
            return Err(format!(
<<<<<<< HEAD
                "Unexpected token expected ':', got {} {}.{}",
=======
                "Unexpected token expected ':' {} {}.{}",
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
                self.current_token, self.current_token.row, self.current_token.col
            ));
        }

        let let_type = self.parse_type()?;

        self.advance_tokens();

        if let TokenType::Assign = self.current_token.token_type {
            self.advance_tokens()
        } else {
            return Err(format!(
<<<<<<< HEAD
                "Unexpected token expected '=', got {} {}.{}",
=======
                "Unexpected token expected '=' {} {}.{}",
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
                self.current_token, self.current_token.row, self.current_token.col
            ));
        }

        let expr = self.parse_expression()?;

        self.advance_tokens();

        Ok(Statement::new_let_statement(identifier, let_type, expr))
    }
<<<<<<< HEAD
    
    fn parse_return(&mut self) -> Result<Statement, String> {
        self.advance_tokens();

        match self.parse_expression() {
            Ok(expr) => Ok(Statement::Return(expr)),
            Err(e) => Err(e),
        }
    }

    fn parse_end(&mut self) -> Result<Statement, String> {
        if let TokenType::NewLine = self.peek_token.token_type {
            self.advance_tokens();
            Ok(Statement::End)
        } else {
            Err(format!("Expected new line, got {} {}.{}", self.peek_token, self.peek_token.row, self.peek_token.col))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
=======

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
        match &self.current_token.token_type {
            TokenType::Str(_)
            | TokenType::Int(_)
            | TokenType::Character(_)
            | TokenType::False
<<<<<<< HEAD
            | TokenType::True => Ok(Expression::Value(mem::take(&mut self.current_token))),
            _ => Err(format!(
                "Unexpected token expected value, got {} {}.{}",
=======
            | TokenType::True => Ok(Expression::Value(
                mem::take(&mut self.current_token))),
            _ => Err(format!(
                "Unexpected token expected value {} {}.{}",
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
                self.current_token, self.current_token.row, self.current_token.col
            )),
        }
    }

<<<<<<< HEAD
    fn parse_type(&mut self) -> Result<Token, String> {
=======
    pub fn parse_type(&mut self) -> Result<Token, String> {
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
        match &self.current_token.token_type {
            TokenType::StringType
            | TokenType::IntType
            | TokenType::CharType
            | TokenType::BoolType => Ok(mem::take(&mut self.current_token)),
            _ => Err(format!(
<<<<<<< HEAD
                "Unexpected token expected type, got {} {}.{}",
=======
                "Unexpected token expected type {} {}.{}",
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
                self.current_token, self.current_token.row, self.current_token.col
            )),
        }
    }

<<<<<<< HEAD
    fn parse_identifier(&mut self) -> Result<Token, String> {
=======
    pub fn parse_identifier(&mut self) -> Result<Token, String> {
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
        if let TokenType::Identity(_) = &self.current_token.token_type {
            Ok(mem::take(&mut self.current_token))
        } else {
            Err(format!(
<<<<<<< HEAD
                "Unexpected token expected Identity, got {} {}.{}",
=======
                "Invalid indtifier {}, expected Identity {}.{}",
>>>>>>> 10d471eb2ade24717232ef856971301e1dc0d428
                self.current_token, self.current_token.row, self.current_token.col
            ))
        }
    }
}
