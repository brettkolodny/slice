use std::str::Chars;
use std::iter::Peekable;
use crate::token::Token; 


pub struct Lexer<'a> {
    position: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { position: input.chars().peekable() }
    }

    pub fn next(&mut self) -> Token {
        let character = self.position.next();

        if let Some(c) = character {
            let token = {
                if c.is_ascii_alphabetic() {
                    let mut word = String::new();
                    word.push(c);

                    self.get_rest_of_word(&mut word);
                    dbg!{&word};

                    match word.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        _ => Token::Identity(word.clone())
                    }

                } else {
                    match c {
                        '=' => Token::Assign,
                        '+' => Token::Plus,
                        '(' => Token::LParen,
                        ')' => Token::RParen,
                        '[' => Token::LBracket,
                        ']' => Token::RBracket,
                        '{' => Token::LBrace,
                        '}' => Token::RBrace,
                        ':' => Token::Colon,
                        ';' => Token::SemiColon,
                        ',' => Token::Comma,
                        ' ' => self.next(),
                        _ => Token::Illegal
                    }
                }
            };
            return token;
        } else {
            return Token::EOF;
        }
    }

    fn get_rest_of_word(&mut self, word: &mut String) { 
        let mut c_peek = self.position.peek();

        while !c_peek.is_none() && Lexer::is_letter(c_peek.unwrap()) {
            word.push(self.position.next().unwrap());
            c_peek = self.position.peek();
        }
    }

    fn is_letter(c: &char) -> bool {
        if c.is_ascii_alphabetic() || c == &'_' {
            return true;
        }

        false
    }
}
