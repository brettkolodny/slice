use crate::lexer::Lexer;
use crate::token::TokenType;

#[test]
fn basic() {
    let input = "=+(){},:;";
    let expected = [
        TokenType::Assign,
        TokenType::Plus,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Colon,
        TokenType::SemiColon,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_int() {
    let input = "let foo = 3";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("foo")),
        TokenType::Assign,
        TokenType::Int(3),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_no_spaces() {
    let input = "let x=3";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("x")),
        TokenType::Assign,
        TokenType::Int(3),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_int_long() {
    let input = "let foo = 1234567890";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("foo")),
        TokenType::Assign,
        TokenType::Int(1234567890),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn function_basic() {
    let input = "fn test_function(argument):
        3
    end";
    let expected = [
        TokenType::Function,
        TokenType::Identity(String::from("test_function")),
        TokenType::LParen,
        TokenType::Identity(String::from("argument")),
        TokenType::RParen,
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::Int(3),
        TokenType::NewLine,
        TokenType::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_with_type() {
    let input = "let int_type: int = 3
        let bool_type: bool = True";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("int_type")),
        TokenType::Colon,
        TokenType::IntType,
        TokenType::Assign,
        TokenType::Int(3),
        TokenType::NewLine,
        TokenType::Let,
        TokenType::Identity(String::from("bool_type")),
        TokenType::Colon,
        TokenType::BoolType,
        TokenType::Assign,
        TokenType::True,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn function_typed_argument() {
    let input = "fn typed_argument(argument_name: bool):
        True
    end";
    let expected = [
        TokenType::Function,
        TokenType::Identity(String::from("typed_argument")),
        TokenType::LParen,
        TokenType::Identity(String::from("argument_name")),
        TokenType::Colon,
        TokenType::BoolType,
        TokenType::RParen,
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::True,
        TokenType::NewLine,
        TokenType::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn function_output() {
    let input = "fn name?(name) -> bool:
        False
    end";
    let expected = [
        TokenType::Function,
        TokenType::Identity(String::from("name?")),
        TokenType::LParen,
        TokenType::Identity(String::from("name")),
        TokenType::RParen,
        TokenType::Output,
        TokenType::BoolType,
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::False,
        TokenType::NewLine,
        TokenType::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn pipe() {
    let input = "let fizzBuzz = argument -> upper -> lower";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("fizzBuzz")),
        TokenType::Assign,
        TokenType::Identity(String::from("argument")),
        TokenType::Output,
        TokenType::Identity(String::from("upper")),
        TokenType::Output,
        TokenType::Identity(String::from("lower")),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn string_assign() {
    let input = "let a_string = \"Hello World!\"";
    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("a_string")),
        TokenType::Assign,
        TokenType::Str(String::from("Hello World!")),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn unclosed_string() {
    let input = "\"Hello world!";
    let expected = [TokenType::Illegal];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn function_complex() {
    let input = "fn add_one(num: int) -> int:
        let new_num: int = num + 1
        new_num
    end";

    let expected = [
        TokenType::Function,
        TokenType::Identity(String::from("add_one")),
        TokenType::LParen,
        TokenType::Identity(String::from("num")),
        TokenType::Colon,
        TokenType::IntType,
        TokenType::RParen,
        TokenType::Output,
        TokenType::IntType,
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::Let,
        TokenType::Identity(String::from("new_num")),
        TokenType::Colon,
        TokenType::IntType,
        TokenType::Assign,
        TokenType::Identity(String::from("num")),
        TokenType::Plus,
        TokenType::Int(1),
        TokenType::NewLine,
        TokenType::Identity(String::from("new_num")),
        TokenType::NewLine,
        TokenType::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn condition_basic() {
    let input = "if x > 3:";

    let expected = [
        TokenType::If,
        TokenType::Identity(String::from("x")),
        TokenType::GreaterThan,
        TokenType::Int(3),
        TokenType::Colon,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
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
        TokenType::If,
        TokenType::Identity(String::from("num")),
        TokenType::Equal,
        TokenType::Int(3),
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::True,
        TokenType::NewLine,
        TokenType::Elif,
        TokenType::Identity(String::from("num")),
        TokenType::LessThan,
        TokenType::Int(3),
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::False,
        TokenType::NewLine,
        TokenType::Else,
        TokenType::Colon,
        TokenType::NewLine,
        TokenType::True,
        TokenType::NewLine,
        TokenType::End,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn negative_number() {
    let input = "let neg = -23456";

    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("neg")),
        TokenType::Assign,
        TokenType::Int(-23456),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn dictionary() {
    let input = "let dict = {arg: True, arg_two: \"two\", \"arg_three\": -3}";

    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("dict")),
        TokenType::Assign,
        TokenType::LBrace,
        TokenType::Identity(String::from("arg")),
        TokenType::Colon,
        TokenType::True,
        TokenType::Comma,
        TokenType::Identity(String::from("arg_two")),
        TokenType::Colon,
        TokenType::Str(String::from("two")),
        TokenType::Comma,
        TokenType::Str(String::from("arg_three")),
        TokenType::Colon,
        TokenType::Int(-3),
        TokenType::RBrace,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn list() {
    let input = "let list = [1, True, \"three\"]";

    let expected = [
        TokenType::Let,
        TokenType::Identity(String::from("list")),
        TokenType::Assign,
        TokenType::LBracket,
        TokenType::Int(1),
        TokenType::Comma,
        TokenType::True,
        TokenType::Comma,
        TokenType::Str(String::from("three")),
        TokenType::RBracket,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn not_equal() {
    let input = "3 != 4";

    let expected = [TokenType::Int(3), TokenType::NotEqual, TokenType::Int(4)];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn correct_column() {
    let input = "let -3
True";

    let expected = [1, 5, 7, 1];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.col, expected[i]);
    }
}

#[test]
fn correct_row() {
    let input = "test
    3 hello
    True
    end";

    let expected = [1, 1, 2, 2, 2, 3, 3, 4];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.row, expected[i]);
    }
}

#[test]
fn array() {
    let input = "[/1, 2, 3/]/[]/";

    let expected = [
        TokenType::LArray,
        TokenType::Int(1),
        TokenType::Comma,
        TokenType::Int(2),
        TokenType::Comma,
        TokenType::Int(3),
        TokenType::RArray,
        TokenType::Divide,
        TokenType::LBracket,
        TokenType::RBracket,
        TokenType::Divide,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}
