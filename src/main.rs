#![allow(dead_code)]
mod lexer;
mod token;
mod ast;
mod parser;

#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io;

fn main() {
    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut lexer = Lexer::new(input.as_str());
                let mut token = lexer.next();

                while token.token_type != TokenType::EOF {
                    print!("{} ", token);
                    token = lexer.next();
                }

                println!();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
