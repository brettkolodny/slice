use crate::parser::Parser;
use crate::lexer::Lexer;

#[test]
fn let_int_basic() {
    let input = "let x: int = 5";
    let mut lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);

    assert!(false);
}
