use compiler::Compiler;

mod compiler;
mod diagnostic;
mod utils;

mod lexer;

mod ast;
mod parser;

fn main() {
    let source = "$";
    let compiler = Compiler::new(source);
    compiler.compile();
}
