use crate::token::Token;
use crate::lexer::Lexer;

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
        Token::SemiColon
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..8 {
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

    for i in 0..4 {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }

    assert!(true);
}

