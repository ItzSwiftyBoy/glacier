use std::{fs::File, io::Read, path::Path};

use clap::Parser;
use compiler::Compiler;
use lexer::Lexer;

mod compiler;
mod diagnostic;
// mod printer;
mod utils;

mod ast;
mod lexer;
mod parser;
// mod symbol;
mod types;

pub fn get_file_content(filepath: &Path) -> String {
    let mut file = match File::open(filepath) {
        Ok(content) => content,
        Err(r) => {
            eprintln!("Couldn't open the file. Reason: {}", r);
            return String::new();
        }
    };
    let mut source = String::new();
    if file.read_to_string(&mut source).is_err() {
        eprintln!("Got an invalid UTF-8 character!");
        return String::new();
    };

    source
}

pub fn get_line_from_index(source: &str, index: usize) -> &str {
    let start = source[..index].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = source[index..]
        .find('\n')
        .map(|i| index + i)
        .unwrap_or(source.len());
    &source[start..end]
}

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

    let tokens = match Lexer::new(&compiler).identify_tokens() {
        Some(x) => x,
        None => return,
    };
    // println!("{:#?}", tokens);
    let mut parser = parser::Parser::new(&compiler, tokens);
    let ast = parser.parse();
    compiler.dump_ast(&ast);
}
