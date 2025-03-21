use crate::{diagnostic::DiagnosticReporter, lexer::Lexer};

pub struct Compiler<'a> {
    source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn compile(&'a self) {
        let mut diagnostics = DiagnosticReporter::new();
        let lexer = Lexer::new(self.source).identify_tokens();
        let tokens = lexer.0;
        for diagnostic in lexer.1 {
            diagnostics.add(diagnostic);
        }
        diagnostics.report(self.source);
        println!("{:#?}", tokens);
    }
}
