use crate::utils::Span;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Hint,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(level: DiagnosticLevel, message: String) -> Self {
        Self {
            level,
            message,
            span: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> Self {
        Self {
            level: self.level,
            message: self.message,
            span: self.span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn report(&self, source: &'static str) {
        for diagnostic in &self.diagnostics {
            match diagnostic.level {
                DiagnosticLevel::Error => eprintln!("Error: {}", diagnostic.message),
                DiagnosticLevel::Warning => eprintln!("Warning: {}", diagnostic.message),
                DiagnosticLevel::Note => eprintln!("Note: {}", diagnostic.message),
                DiagnosticLevel::Hint => eprintln!("Hint: {}", diagnostic.message),
            }

            if let Some(span) = &diagnostic.span {
                // Extract the relevant line and column from the source code
                let (line, column) = self.get_line_and_column(source, span.start);
                let line_content = self.get_line_content(source, span.start);

                eprintln!("  at line {}, column {}", line, column);
                eprintln!("  |");
                eprintln!("{} | {}", line, line_content);
                eprintln!("  | {:>width$}", "^", width = column + 1);
            }
        }
    }

    // Helper function to calculate line and column from a span's start index
    fn get_line_and_column(&self, source: &str, index: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for (i, ch) in source.chars().enumerate() {
            if i == index {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    // Helper function to get the content of the line containing the span
    fn get_line_content(&self, source: &'static str, index: usize) -> &'static str {
        let start = source[..index].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let end = source[index..]
            .find('\n')
            .map(|i| index + i)
            .unwrap_or(source.len());
        &source[start..end]
    }
}
