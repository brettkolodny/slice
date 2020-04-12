#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Illegal,
    EOF,
    Identity(String),
    Int(isize),
    Str(String),
    Character(char),
    True,
    False,
    Assign,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Plus,
    If,
    Else,
    Elif,
    Minus,
    Divide,
    Comma,
    SemiColon,
    Colon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LArray,
    RArray,
    Function,
    Let,
    End,
    Pin,
    NewLine,
    StringType,
    BoolType,
    IntType,
    CharType,
    Output,
}

pub struct Token {
    pub token_type: TokenType,
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new(token_type: TokenType, row: u32, col: u32) -> Self {
        Token {
            token_type,
            row,
            col,
        }
    }
}
