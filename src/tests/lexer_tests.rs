use crate::lexer::Lexer;
use crate::token::TokenType::*;

#[test]
fn basic() {
    let input = "=+(){},:;";
    let expected = [
        Assign, Plus, LParen, RParen, LBrace, RBrace, Comma, Colon, SemiColon,
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
    let expected = [Let, Identity(String::from("foo")), Assign, Int(3)];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_no_spaces() {
    let input = "let x=3";
    let expected = [Let, Identity(String::from("x")), Assign, Int(3)];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn assign_int_long() {
    let input = "let foo = 1234567890";
    let expected = [Let, Identity(String::from("foo")), Assign, Int(1234567890)];

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
        Function,
        Identity(String::from("test_function")),
        LParen,
        Identity(String::from("argument")),
        RParen,
        Colon,
        NewLine,
        Int(3),
        NewLine,
        End,
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
        Let,
        Identity(String::from("int_type")),
        Colon,
        IntType,
        Assign,
        Int(3),
        NewLine,
        Let,
        Identity(String::from("bool_type")),
        Colon,
        BoolType,
        Assign,
        True,
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
        Function,
        Identity(String::from("typed_argument")),
        LParen,
        Identity(String::from("argument_name")),
        Colon,
        BoolType,
        RParen,
        Colon,
        NewLine,
        True,
        NewLine,
        End,
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
        Function,
        Identity(String::from("name?")),
        LParen,
        Identity(String::from("name")),
        RParen,
        Output,
        BoolType,
        Colon,
        NewLine,
        False,
        NewLine,
        End,
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
        Let,
        Identity(String::from("fizzBuzz")),
        Assign,
        Identity(String::from("argument")),
        Output,
        Identity(String::from("upper")),
        Output,
        Identity(String::from("lower")),
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
        Let,
        Identity(String::from("a_string")),
        Assign,
        Str(String::from("Hello World!")),
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn string_unclosed() {
    let input = "\"Hello world!";
    let expected = Illegal;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next().token_type, expected);
}

#[test]
fn string_empty() {
    let input = "\"\"";
    let expected = Str(String::from(""));

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next().token_type, expected);
}

#[test]
fn string_within_string() {
    let input = "\"\"\"";
    let expected = Str(String::from("\""));

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next().token_type, expected);
}

#[test]
fn function_unused_argument() {
    let input = "fn unused(_) -> bool:
        True
    end";

    let expected = [
        Function,
        Identity(String::from("unused")),
        LParen,
        Identity(String::from("_")),
        RParen,
        Output,
        BoolType,
        Colon,
        NewLine,
        True,
        NewLine,
        End,
    ];

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
        Function,
        Identity(String::from("add_one")),
        LParen,
        Identity(String::from("num")),
        Colon,
        IntType,
        RParen,
        Output,
        IntType,
        Colon,
        NewLine,
        Let,
        Identity(String::from("new_num")),
        Colon,
        IntType,
        Assign,
        Identity(String::from("num")),
        Plus,
        Int(1),
        NewLine,
        Identity(String::from("new_num")),
        NewLine,
        End,
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

    let expected = [If, Identity(String::from("x")), GreaterThan, Int(3), Colon];

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
        If,
        Identity(String::from("num")),
        Equal,
        Int(3),
        Colon,
        NewLine,
        True,
        NewLine,
        Elif,
        Identity(String::from("num")),
        LessThan,
        Int(3),
        Colon,
        NewLine,
        False,
        NewLine,
        Else,
        Colon,
        NewLine,
        True,
        NewLine,
        End,
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

    let expected = [Let, Identity(String::from("neg")), Assign, Int(-23456)];

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
        Let,
        Identity(String::from("dict")),
        Assign,
        LBrace,
        Identity(String::from("arg")),
        Colon,
        True,
        Comma,
        Identity(String::from("arg_two")),
        Colon,
        Str(String::from("two")),
        Comma,
        Str(String::from("arg_three")),
        Colon,
        Int(-3),
        RBrace,
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
        Let,
        Identity(String::from("list")),
        Assign,
        LBracket,
        Int(1),
        Comma,
        True,
        Comma,
        Str(String::from("three")),
        RBracket,
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

    let expected = [Int(3), NotEqual, Int(4)];

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
        LArray,
        Int(1),
        Comma,
        Int(2),
        Comma,
        Int(3),
        RArray,
        Divide,
        LBracket,
        RBracket,
        Divide,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn pin() {
    let input = "[1, ^x] = [1, 2]";

    let expected = [
        LBracket,
        Int(1),
        Comma,
        Pin,
        Identity(String::from("x")),
        RBracket,
        Assign,
        LBracket,
        Int(1),
        Comma,
        Int(2),
        RBracket,
    ];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i])
    }
}

#[test]
fn character() {
    let input = "'a' '3' ''' 'abc'";

    let expected = [Character('a'), Character('3'), Character('\''), Illegal];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.token_type, expected[i]);
    }
}

#[test]
fn column_character() {
    let input = "'a' '3' '123' ''' 'abc' end";

    let expected = [1, 5, 9, 15, 19, 25];

    let mut lexer = Lexer::new(input);

    for i in 0..expected.len() {
        let token = lexer.next();
        assert_eq!(token.col, expected[i]);
    }
}

