use crate::{diagnostic::DiagnosticReporter, lexer::Lexer};

pub struct Compiler<'a> {
    source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn compile(&self) {
        let mut lexer = Lexer::new(self.source);
        let tokens = lexer.next_token();
        println!("{:#?}", tokens);
    }
}
