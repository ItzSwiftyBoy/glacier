use crate::{compiler::Compiler, utils::Span};
use colored::Colorize;

#[macro_export]
macro_rules! diag {
    ( $kind:expr, $p_msg:expr, $s_msg:expr, $span:expr ) => {
        Diagnostic::new($kind, String::from($p_msg), $s_msg, $span)
    };
    ( $p_msg:expr, $span:expr ) => {
        diag!(DiagnosticKind::Error, $p_msg, None, $span)
    };
    ( $p_msg:expr, $s_msg:expr, $span:expr ) => {
        diag!(
            DiagnosticKind::Error,
            $p_msg,
            Some(String::from($s_msg)),
            $span
        )
    };
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub primary_msg: String,
    pub secondary_msg: Option<String>,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(
        kind: DiagnosticKind,
        primary_msg: String,
        secondary_msg: Option<String>,
        span: Span,
    ) -> Self {
        Self {
            kind,
            primary_msg,
            secondary_msg,
            span,
        }
    }

    pub fn with_secondary_msg(mut self, msg: String) -> Self {
        self.secondary_msg = Some(msg);
        self
    }

    /* pub fn with_note(mut self, note: String) -> Self {
        self.note.push(note);
        self
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint.push(hint);
        self
    } */

    pub fn print(&self, compiler: &Compiler) {
        match self.kind {
            DiagnosticKind::Error => {
                eprintln!(
                    "{}: {}",
                    "Error".red().bold(),
                    self.primary_msg.bright_white().bold()
                )
            }
            DiagnosticKind::Warning => eprintln!("Warning: {}", self.primary_msg),
        }

        let span = &self.span;
        let source = &Compiler::get_file_source(compiler.get_module_filepath(span.file_id));
        let (line, column) = self.get_line_and_column(source, span.start);
        let line_content = {
            let start = source[..span.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let end = source[span.start..]
                .find('\n')
                .map(|i| span.start + i)
                .unwrap_or(source.len());
            &source[start..end]
        };
        let secondary_msg = match &self.secondary_msg {
            Some(msg) => &format!("{}", msg),
            None => "",
        };

        eprintln!(
            "\t{}",
            format!(
                "--> {} {}:{}",
                compiler.get_module_filepath(span.file_id).display(),
                line,
                column
            )
            .bright_green()
            .bold()
        );
        eprintln!("{}", format!("  |").cyan().bold());
        eprintln!("{}  {}", format!("{} |", line).cyan().bold(), line_content);
        eprintln!(
            "{}",
            format!("  |  {:>width$} {}", "^", secondary_msg, width = column)
                .cyan()
                .bold()
        );
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
}

#[derive(Debug)]
pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
    error: u32,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            error: 0,
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        if diagnostic.kind == DiagnosticKind::Error {
            self.error += 1;
        }
        self.diagnostics.push(diagnostic);
    }

    pub fn has_error(&self) -> bool {
        self.error != 0
    }

    pub fn report(&self, compiler: &Compiler) {
        for diagnostic in &self.diagnostics {
            diagnostic.print(compiler);
        }
        if self.error < 1 {
            eprintln!(
                "{}",
                format!("{} errors have been emitted.", self.error)
                    .bright_white()
                    .bold()
            )
        } else {
            eprintln!(
                "{}",
                format!("{} error has been emitted.", self.error)
                    .bright_white()
                    .bold()
            )
        }
    }
}
