use clap::{Arg, Command, Parser};
use compiler::Compiler;
use lexer::Lexer;

mod compiler;
mod diagnostic;
mod printer;
mod utils;

mod ast;
mod lexer;
mod parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct OliveArgs {
    #[arg(value_name = "FILE", required = true)]
    file: String,

    #[arg(long)]
    dump_ast: bool,
}

fn main() {
    // let cmd = Command::new("glacier").arg(Arg::new("file").required(true));
    // args.get_matches()
    //     .get_one::<String>("file")
    //     .expect("Expected a filename!"),
    let args = OliveArgs::parse();

    let compiler = Compiler::new(&args.file, args.dump_ast);

    let tokens = Lexer::new(&compiler).identify_tokens();
    // // println!("{:#?}", tokens);
    let mut parser = parser::Parser::new(&compiler, tokens);
    let ast = parser.parse();
    compiler.dump_ast(ast);
    compiler.print_error();
}
