use clap::{Arg, Command};
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use std::{fs::File, io::Read};

mod compiler;
mod diagnostic;
mod utils;

mod lexer;

mod ast;
mod parser;

fn main() {
    let cmd = Command::new("glacier").arg(Arg::new("file").required(true));

    let mut file = match File::open(
        cmd.get_matches()
            .get_one::<String>("file")
            .expect("Expected a filename!"),
    ) {
        Ok(content) => content,
        Err(r) => {
            eprintln!("Couldn't open the file. Reason: {}", r);
            return;
        }
    };

    let mut source = String::new();
    if file.read_to_string(&mut source).is_err() {
        eprintln!("Got an invalid UTF-8 character!");
        return;
    };

    let compiler = Compiler::new(&source);
    let lexer = Lexer::new(compiler.source);
    let tokens = lexer.identify_tokens();
    println!("{:#?}", tokens);
    let mut parser = Parser::new(&compiler, tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);
}
