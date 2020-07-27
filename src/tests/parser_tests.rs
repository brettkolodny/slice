use crate::ast::{Expression, Statement};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType;

#[test]
fn let_int_basic() {
    let input = "let x: int = 5";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::Let(ls) = stmnt {
        assert_eq!(ls.name.token_type, TokenType::Identity(String::from("x")));
        assert_eq!(ls.var_type.token_type, TokenType::IntType);
    }
}

#[test]
fn let_string_basic() {
    let input = "let s: string = \"slice\"";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::Let(ls) = stmnt {
        assert_eq!(ls.name.token_type, TokenType::Identity(String::from("s")));
        assert_eq!(ls.var_type.token_type, TokenType::StringType);
    }
}

#[test]
fn return_value_int() {
    let input = "return 3";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::Return(_e) = stmnt {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn end_test_valid() {
    let input = "end\n";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::End = stmnt {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn prefix_test() {
    let input = "!True";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::Expr(v) = stmnt {
        assert!(true);
    } else {
        assert!(false);
    }

    let input = "-foo";
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    let stmnt = &ast[0];

    if let Statement::Expr(v) = stmnt {
        assert!(true);
    } else {
        assert!(false);
    }
}
