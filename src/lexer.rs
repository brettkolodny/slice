use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    position: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            position: input.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Token {
        let character = self.position.next();

        if let Some(c) = character {
            if c.is_ascii_alphabetic() {
                let mut word = String::new();
                word.push(c);

                self.get_rest_of_word(&mut word);

                match word.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    "end" => Token::End,
                    "string" => Token::StringType,
                    "bool" => Token::BoolType,
                    "int" => Token::IntType,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "elif" => Token::Elif,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "True" => Token::True,
                    "False" => Token::False,
                    _ => Token::Identity(word.clone()),
                }
            } else if c.is_ascii_digit() {
                let mut int = String::new();
                int.push(c);

                self.get_rest_of_int(&mut int);

                Token::Int(int.parse::<isize>().unwrap())
            } else {
                match c {
                    '=' => self.get_assign_or_equal(),
                    '+' => Token::Plus,
                    '-' => self.minus_or_pipe_or_negative(),
                    '!' => self.get_not_or_not_equal(),
                    '>' => Token::GreaterThan,
                    '<' => Token::LessThan,
                    '/' => Token::Divide,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '[' => Token::LBracket,
                    ']' => Token::RBracket,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    ':' => Token::Colon,
                    ';' => Token::SemiColon,
                    ',' => Token::Comma,
                    '"' => self.get_string(),
                    '\n' => Token::NewLine,
                    '\t' => self.next(),
                    ' ' => self.next(),
                    _ => Token::Illegal,
                }
            }
        } else {
            Token::EOF
        }
    }

    fn get_not_or_not_equal(&mut self) -> Token {
        let c_peek = self.position.peek();

        if c_peek.is_some() && c_peek == Some(&'=') {
            self.position.next();
            return Token::NotEqual;
        }

        Token::Not
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
            return Token::Output;
        } else if self.position.peek().is_some() {
            let next_c = self.position.peek().unwrap();
            if next_c.is_ascii_digit() {
                let mut negative_number = String::from("-");

                self.get_rest_of_int(&mut negative_number);

                return Token::Int(negative_number.parse::<isize>().unwrap());
            }
        }

        Token::Minus
    }

    fn get_assign_or_equal(&mut self) -> Token {
        if self.position.peek() == Some(&'=') {
            self.position.next();
            return Token::Equal;
        }

        Token::Assign
    }

    fn get_string(&mut self) -> Token {
        let mut string = String::from("");

        let mut character = self.position.next();

        while character.is_some() && character != Some('"') {
            string.push(character.unwrap());

            character = self.position.next();

            if character.is_none() {
                return Token::Illegal;
            }
        }

        Token::Str(string)
    }
}
