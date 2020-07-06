use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    TMP,
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
    Return,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::TMP
    }
}

#[derive(Clone, Default, Debug)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}
