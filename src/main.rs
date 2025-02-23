mod utils;

mod lexer;
use lexer::Lexer;

mod ast;
mod parser;

fn main() {
    let mut lexer = Lexer::new("var mut x = 36..6");
    let tokens = lexer.next_token();
    println!("{:#?}", tokens);
}
