use std::{cell::RefCell, rc::Rc};

use compiler::Compiler;
use lexer::Lexer;

mod compiler;
mod diagnostic;
mod utils;

mod lexer;

mod ast;
mod parser;

fn main() {
    let source = "var x = 5";
    let compiler: Rc<RefCell<Compiler<'_>>> =
        Rc::new(RefCell::new(Compiler::new(source.as_bytes())));
    let tokens = Lexer::new(compiler.clone()).identify_tokens();
    println!("{:#?}", tokens);
    compiler.borrow().reporter.report(source);
}
