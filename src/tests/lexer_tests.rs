use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn lexer_test_basic() {
    let input = "=+(){},:;";
    let expected = [
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Colon,
        Token::SemiColon,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_test_assign_int() {
    let input = "let foo = 3";
    let expected = [
        Token::Let,
        Token::Identity(String::from("foo")),
        Token::Assign,
        Token::Int(3),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_test_assign_int_long() {
    let input = "let foo = 1234567890";
    let expected = [
        Token::Let,
        Token::Identity(String::from("foo")),
        Token::Assign,
        Token::Int(1234567890),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_test_functions() {
    let input = "fn test_function(argument):
        3
    end";
    let expected = [
        Token::Function,
        Token::Identity(String::from("test_function")),
        Token::LParen,
        Token::Identity(String::from("argument")),
        Token::RParen,
        Token::Colon,
        Token::NewLine,
        Token::Int(3),
        Token::NewLine,
        Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_test_assign_with_type() {
    let input = "let int_type: int = 3
        let bool_type: bool = True";
    let expected = [
        Token::Let,
        Token::Identity(String::from("int_type")),
        Token::Colon,
        Token::IntType,
        Token::Assign,
        Token::Int(3),
        Token::NewLine,
        Token::Let,
        Token::Identity(String::from("bool_type")),
        Token::Colon,
        Token::BoolType,
        Token::Assign,
        Token::True,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_test_function_typed_argument() {
    let input = "fn typed_argument(argument_name: bool):
        True
    end";
    let expected = [
        Token::Function,
        Token::Identity(String::from("typed_argument")),
        Token::LParen,
        Token::Identity(String::from("argument_name")),
        Token::Colon,
        Token::BoolType,
        Token::RParen,
        Token::Colon,
        Token::NewLine,
        Token::True,
        Token::NewLine,
        Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexter_test_function_output() {
    let input = "fn name?(name) -> bool:
        False
    end";
    let expected = [
        Token::Function,
        Token::Identity(String::from("name?")),
        Token::LParen,
        Token::Identity(String::from("name")),
        Token::RParen,
        Token::Output,
        Token::BoolType,
        Token::Colon,
        Token::NewLine,
        Token::False,
        Token::NewLine,
        Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

#[test]
fn lexer_pipe_test() {
    let input = "let fizzBuzz = argument -> upper -> lower";
    let expected = [
        Token::Let,
        Token::Identity(String::from("fizzBuzz")),
        Token::Assign,
        Token::Identity(String::from("argument")),
        Token::Output,
        Token::Identity(String::from("upper")),
        Token::Output,
        Token::Identity(String::from("lower")),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..(expected.len()) {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}
