use clap::{Arg, Command};
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;

mod compiler;
mod diagnostic;
mod utils;

mod lexer;

mod ast;
mod parser;

fn main() {
    let cmd = Command::new("glacier").arg(Arg::new("file").required(true));

    let compiler = Compiler::new(
        cmd.get_matches()
            .get_one::<String>("file")
            .expect("Expected a filename!"),
    );

    let tokens = Lexer::new(&compiler).identify_tokens();
    // // println!("{:#?}", tokens);
    let mut parser = Parser::new(&compiler, tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);
    compiler.print_error();
}
