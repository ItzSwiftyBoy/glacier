use crate::diagnostic::DiagnosticReporter;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Compiler<'a> {
    pub source: &'a [u8],
    pub reporter: DiagnosticReporter,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            reporter: DiagnosticReporter::new(),
        }
    }
}
