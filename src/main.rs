use std::{cell::RefCell, fs::File, rc::Rc};

use command::*;
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;

mod compiler;
mod diagnostic;
mod utils;

mod lexer;

mod ast;
mod command;
mod parser;

fn main() {
    args_parse();
    let source = "x";
    let compiler: Rc<RefCell<Compiler<'_>>> =
        Rc::new(RefCell::new(Compiler::new(source.as_bytes())));
    let tokens = Lexer::new(compiler.clone()).identify_tokens();
    let mut parser = Parser::new(compiler.clone(), tokens.clone());
    let ast = parser.parse();
    println!("{:#?}", tokens);
    println!("{:#?}", ast);
    compiler.borrow().reporter.report(source);
}
