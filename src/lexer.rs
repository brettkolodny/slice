use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    position: Peekable<Chars<'a>>,
    row: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            position: input.chars().peekable(),
            row: 1,
            col: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        let character = self.position.next();

        self.col += 1;

        if let Some(c) = character {
            if c.is_ascii_alphabetic() || c == '_' {
                let mut word = String::new();
                word.push(c);

                self.get_rest_of_word(&mut word);

                let start_col = self.col;
                self.col += (word.len() - 1) as u32;

                match word.as_str() {
                    "let" => Token::new(TokenType::Let, self.row, start_col),
                    "fn" => Token::new(TokenType::Function, self.row, start_col),
                    "end" => Token::new(TokenType::End, self.row, start_col),
                    "string" => Token::new(TokenType::StringType, self.row, start_col),
                    "bool" => Token::new(TokenType::BoolType, self.row, start_col),
                    "int" => Token::new(TokenType::IntType, self.row, start_col),
                    "if" => Token::new(TokenType::If, self.row, start_col),
                    "else" => Token::new(TokenType::Else, self.row, start_col),
                    "elif" => Token::new(TokenType::Elif, self.row, start_col),
                    "and" => Token::new(TokenType::And, self.row, start_col),
                    "or" => Token::new(TokenType::Or, self.row, start_col),
                    "True" => Token::new(TokenType::True, self.row, start_col),
                    "False" => Token::new(TokenType::False, self.row, start_col),
                    "return" => Token::new(TokenType::Return, self.row, start_col),
                    _ => Token::new(TokenType::Identity(word.clone()), self.row, start_col),
                }
            } else if c.is_ascii_digit() {
                let mut int = String::new();
                int.push(c);

                self.get_rest_of_int(&mut int);

                self.col += (int.len() - 1) as u32;

                Token::new(
                    TokenType::Int(int.parse::<isize>().unwrap()),
                    self.row,
                    self.col,
                )
            } else {
                match c {
                    '=' => self.get_assign_or_equal(),
                    '+' => Token::new(TokenType::Plus, self.row, self.col),
                    '-' => self.minus_or_pipe_or_negative(),
                    '!' => self.get_not_or_not_equal(),
                    '>' => Token::new(TokenType::GreaterThan, self.row, self.col),
                    '<' => Token::new(TokenType::LessThan, self.row, self.col),
                    '/' => self.get_slash_or_array(),
                    '(' => Token::new(TokenType::LParen, self.row, self.col),
                    ')' => Token::new(TokenType::RParen, self.row, self.col),
                    '[' => self.get_lbracket_or_array(),
                    ']' => Token::new(TokenType::RBracket, self.row, self.col),
                    '{' => Token::new(TokenType::LBrace, self.row, self.col),
                    '}' => Token::new(TokenType::RBrace, self.row, self.col),
                    ':' => Token::new(TokenType::Colon, self.row, self.col),
                    ';' => Token::new(TokenType::SemiColon, self.row, self.col),
                    ',' => Token::new(TokenType::Comma, self.row, self.col),
                    '"' => self.get_string(),
                    '^' => Token::new(TokenType::Pin, self.row, self.col),
                    '#' => self.token_after_comment(),
                    '\'' => self.get_character(self.col),
                    '\n' => {
                        let row = self.row;
                        let col = self.col;

                        self.row += 1;
                        self.col = 0;

                        Token::new(TokenType::NewLine, row, col)
                    }
                    '\t' => self.next(),
                    ' ' => self.next(),
                    _ => Token::new(TokenType::Illegal, self.row, self.col),
                }
            }
        } else {
            Token::new(TokenType::EOF, self.row, self.col)
        }
    }

    fn get_not_or_not_equal(&mut self) -> Token {
        let c_peek = self.position.peek();

        if c_peek.is_some() && c_peek == Some(&'=') {
            self.position.next();
            return Token::new(TokenType::NotEqual, self.row, self.col);
        }

        Token::new(TokenType::Not, self.row, self.col)
    }

    fn get_rest_of_word(&mut self, word: &mut String) {
        let mut c_peek = self.position.peek();

        while c_peek.is_some() && Lexer::is_letter(*c_peek.unwrap()) {
            word.push(self.position.next().unwrap());
            c_peek = self.position.peek();
        }
    }

    fn get_rest_of_int(&mut self, word: &mut String) {
        let mut c_peek = self.position.peek();

        while c_peek.is_some() && c_peek.unwrap().is_ascii_digit() {
            word.push(self.position.next().unwrap());
            c_peek = self.position.peek();
        }
    }

    fn is_letter(c: char) -> bool {
        if c.is_ascii_alphabetic() || c == '_' || c == '?' {
            return true;
        }

        false
    }

    fn minus_or_pipe_or_negative(&mut self) -> Token {
        if self.position.peek() == Some(&'>') {
            self.next();
            return Token::new(TokenType::Output, self.row, self.col);
        } else if self.position.peek().is_some() {
            let next_c = self.position.peek().unwrap();
            if next_c.is_ascii_digit() {
                let mut negative_number = String::from("-");

                self.get_rest_of_int(&mut negative_number);

                let start_col = self.col;

                self.col += (negative_number.len() - 1) as u32;

                return Token::new(
                    TokenType::Int(negative_number.parse::<isize>().unwrap()),
                    self.row,
                    start_col,
                );
            }
        }

        Token::new(TokenType::Minus, self.row, self.col)
    }

    fn get_assign_or_equal(&mut self) -> Token {
        if self.position.peek() == Some(&'=') {
            self.position.next();
            return Token::new(TokenType::Equal, self.row, self.col);
        }

        Token::new(TokenType::Assign, self.row, self.col)
    }

    fn get_string(&mut self) -> Token {
        let mut string = String::from("");

        let mut character = self.position.next();

        while character.is_some() && character != Some('"') {
            string.push(character.unwrap());

            character = self.position.next();

            if character.is_none() {
                return Token::new(TokenType::Illegal, self.row, self.col);
            }
        }

        Token::new(TokenType::Str(string), self.row, self.col)
    }

    fn get_character(&mut self, col: u32) -> Token {
        let mut character = String::from("");

        let mut c = self.position.next();
        self.col += 1;

        while c.is_some() && c != Some('\'') {
            character.push(c.unwrap());

            c = self.position.next();
            self.col += 1;

            if c.is_none() {
                return Token::new(TokenType::Illegal, self.row, col);
            }
        }

        if character.is_empty()
            && c.is_some()
            && c == Some('\'')
            && self.position.peek() == Some(&'\'')
        {
            self.position.next();
            self.col += 1;
            return Token::new(TokenType::Character('\''), self.row, col);
        }

        if character.len() > 1 || character.is_empty() {
            return Token::new(TokenType::Illegal, self.row, col);
        }

        Token::new(
            TokenType::Character(character.pop().unwrap()),
            self.row,
            col,
        )
    }

    fn get_lbracket_or_array(&mut self) -> Token {
        let c_peek = self.position.peek();

        if c_peek.is_some() && c_peek == Some(&'/') {
            self.position.next();
            return Token::new(TokenType::LArray, self.row, self.col - 1);
        }

        Token::new(TokenType::LBracket, self.row, self.col)
    }

    fn get_slash_or_array(&mut self) -> Token {
        let c_peek = self.position.peek();

        if c_peek.is_some() && c_peek == Some(&']') {
            self.position.next();
            return Token::new(TokenType::RArray, self.row, self.col - 1);
        }

        Token::new(TokenType::Divide, self.row, self.col)
    }

    fn token_after_comment(&mut self) -> Token {
        let mut character = self.position.next();

        while character.is_some() && character != Some('\n') {
            character = self.position.next();
        }

        if character.is_none() {
            Token::new(TokenType::EOF, self.row, self.col)
        } else {
            let token = Token::new(TokenType::NewLine, self.row, self.col);
            self.row += 1;
            self.col = 0;
            token
        }
    }
}
