use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn basic() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn assign_int() {
    let input = "let foo = 3";
    let expected = [
        Token::Let,
        Token::Identity(String::from("foo")),
        Token::Assign,
        Token::Int(3),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn assign_no_spaces() {
    let input = "let x=3";
    let expected = [
        Token::Let,
        Token::Identity(String::from("x")),
        Token::Assign,
        Token::Int(3),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn assign_int_long() {
    let input = "let foo = 1234567890";
    let expected = [
        Token::Let,
        Token::Identity(String::from("foo")),
        Token::Assign,
        Token::Int(1234567890),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn function_basic() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn assign_with_type() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn function_typed_argument() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn function_output() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn pipe() {
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

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn string_assign() {
    let input = "let a_string = \"Hello World!\"";
    let expected = [
        Token::Let,
        Token::Identity(String::from("a_string")),
        Token::Assign,
        Token::Str(String::from("Hello World!")),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn unclosed_string() {
    let input = "\"Hello world!";
    let expected = [Token::Illegal];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn function_complex() {
    let input = "fn add_one(num: int) -> int:
        let new_num: int = num + 1
        new_num
    end";

    let expected = [
        Token::Function,
        Token::Identity(String::from("add_one")),
        Token::LParen,
        Token::Identity(String::from("num")),
        Token::Colon,
        Token::IntType,
        Token::RParen,
        Token::Output,
        Token::IntType,
        Token::Colon,
        Token::NewLine,
        Token::Let,
        Token::Identity(String::from("new_num")),
        Token::Colon,
        Token::IntType,
        Token::Assign,
        Token::Identity(String::from("num")),
        Token::Plus,
        Token::Int(1),
        Token::NewLine,
        Token::Identity(String::from("new_num")),
        Token::NewLine,
        Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn condition_basic() {
    let input = "if x > 3:";

    let expected = [
        Token::If,
        Token::Identity(String::from("x")),
        Token::GreaterThan,
        Token::Int(3),
        Token::Colon,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn condition_complex() {
    let input = "if num == 3:
        True
    elif num < 3:
        False
    else:
        True
    end";

    let expected = [
        Token::If,
        Token::Identity(String::from("num")),
        Token::Equal,
        Token::Int(3),
        Token::Colon,
        Token::NewLine,
        Token::True,
        Token::NewLine,
        Token::Elif,
        Token::Identity(String::from("num")),
        Token::LessThan,
        Token::Int(3),
        Token::Colon,
        Token::NewLine,
        Token::False,
        Token::NewLine,
        Token::Else,
        Token::Colon,
        Token::NewLine,
        Token::True,
        Token::NewLine,
        Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn negative_number() {
    let input = "let neg = -23456";

    let expected = [
        Token::Let,
        Token::Identity(String::from("neg")),
        Token::Assign,
        Token::Int(-23456),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn dictionary() {
    let input = "let dict = {arg: True, arg_two: \"two\", \"arg_three\": -3}";

    let expected = [
        Token::Let,
        Token::Identity(String::from("dict")),
        Token::Assign,
        Token::LBrace,
        Token::Identity(String::from("arg")),
        Token::Colon,
        Token::True,
        Token::Comma,
        Token::Identity(String::from("arg_two")),
        Token::Colon,
        Token::Str(String::from("two")),
        Token::Comma,
        Token::Str(String::from("arg_three")),
        Token::Colon,
        Token::Int(-3),
        Token::RBrace,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn list() {
    let input = "let list = [1, True, \"three\"]";

    let expected = [
        Token::Let,
        Token::Identity(String::from("list")),
        Token::Assign,
        Token::LBracket,
        Token::Int(1),
        Token::Comma,
        Token::True,
        Token::Comma,
        Token::Str(String::from("three")),
        Token::RBracket,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}

#[test]
fn not_equal() {
    let input = "3 != 4";

    let expected = [Token::Int(3), Token::NotEqual, Token::Int(4)];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token, expected[i]);
    }
}
