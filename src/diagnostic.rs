use crate::{compiler::Compiler, get_file_content, get_line_from_index, utils::Span};
use colored::Colorize;

#[macro_export]
macro_rules! diag {
    ( $kind:expr, $p_msg:expr, $s_msg:expr, $span:expr ) => {
        Diagnostic::new($kind, $span)
            .with_primary_msg($p_msg)
            .with_secondary_msg($s_msg)
    };
    ( $p_msg:expr, $span:expr ) => {
        diag!(DiagnosticKind::Error, $p_msg, String::new(), $span)
    };
    ( $p_msg:expr, $s_msg:expr, $span:expr ) => {
        diag!(DiagnosticKind::Error, $p_msg, $s_msg, $span)
    };
}

/// Helper function to calculate line and column from a `span`'s `start`or `end` index.
fn get_line_and_column(source: &str, index: usize) -> (usize, usize) {
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

#[derive(Debug, PartialEq, Default)]
pub enum DiagnosticKind {
    #[default]
    Error,
    Warning,
    Hint(String), // 0 = The character that needs to be added
}

#[derive(Debug, Default)]
struct Header {
    pub level: DiagnosticKind,
    pub msg: String,
    pub span: Span,
}

impl Header {
    pub fn print(&self, compiler: &Compiler, span: Span) {
        match self.level {
            DiagnosticKind::Error => {
                println!("{}: {}", "Error".bright_red().bold(), &self.msg)
            }
            _ => unimplemented!(),
        }
        let path = span.get_filename(compiler);
        let (line, column) = get_line_and_column(&get_file_content(path), span.start);
        print!("--> {} ", path.display());
        if span.len() == 1 {
            println!("[{}:{}]", line, column);
        } else {
            println!("[{}:{}-{}]", line, column, (column + span.len() - 1));
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Default)]
struct SourceWithMsg<'a> {
    line: &'a str,
    msg: Vec<String>,
    span: Vec<Span>,
}

impl<'a> SourceWithMsg<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line: get_line_from_index(source, span.start),
            msg: vec![],
            span: vec![],
        }
    }

    pub fn error(&mut self, msg: impl Into<String>, span: Span) {
        if !self.msg.is_empty() && !self.span.is_empty() {
            panic!("Cannot push `String` in `Vec` for the first(index 0) element of the `Vec` can be error message.");
        }
        self.msg.push(msg.into());
        self.span.push(span);
    }

    pub fn note(&mut self, msg: impl Into<String>, span: Span) {
        self.msg.push(msg.into());
        self.span.push(span);
    }

    pub fn print(self) {
        println!("{}", "  |".bright_purple());
        println!("{} {}", , self.line);
    }
}

#[derive(Debug, PartialEq, Default)]
struct Annotation {
    pub note: String,
    pub hint: String,
    pub span: Span,
}

impl Annotation {
    pub fn print() {}
}

#[derive(Debug, Default)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    primary_msg: String,
    secondary_msg: String,
    context: Option<(String, Span)>,
    span: Span,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, span: Span) -> Self {
        Self {
            kind,
            span,
            ..Default::default()
        }
    }

    pub fn with_primary_msg(mut self, msg: impl Into<String>) -> Self {
        self.primary_msg = msg.into();
        self
    }

    pub fn with_secondary_msg(mut self, msg: impl Into<String>) -> Self {
        self.secondary_msg = msg.into();
        self
    }

    pub fn with_context(mut self, context: String, span: Span) -> Self {
        self.context = Some((context, span));
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

    fn print_context(&self, source: &str, context: &(String, Span)) {
        let (line, column) = get_line_and_column(source, context.1.end);
        let line_content = {
            let start = source[..context.1.start]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            let end = source[context.1.start..]
                .find('\n')
                .map(|i| context.1.start + i)
                .unwrap_or(source.len());
            &source[start..end]
        };

        eprintln!("{}", "  |".purple().bold());

        eprintln!(
            "{}  {}",
            format!("{} |", line).purple().bold(),
            line_content
        );
        eprintln!(
            "{}",
            format!(
                "  |  {:>width$}{} {}",
                "",
                format!("{:^<width$}", "^", width = context.1.len()),
                self.secondary_msg,
                width = column - 1,
            )
            .purple()
            .bold()
        );
    }

    fn print(&self, compiler: &Compiler) {
        match self.kind {
            DiagnosticKind::Error => {
                eprintln!(
                    "{}: {}",
                    "Error".red().bold(),
                    self.primary_msg.bright_white().bold()
                )
            }
            DiagnosticKind::Warning => eprintln!("Warning: {}", self.primary_msg),
            DiagnosticKind::Hint(_) => {
                eprintln!(
                    "{}: {}",
                    "Hint".green().bold(),
                    self.primary_msg.bright_white().bold()
                )
            }
        }

        let context = &self.context;
        let span = &self.span;
        let source = &get_file_content(span.get_filename(compiler));
        let (line, column) = get_line_and_column(source, span.end);
        let line_content = get_line_from_index(source, span.start);

        let span_end = if span.len() == 1 {
            String::new()
        } else {
            format!("-{}", get_line_and_column(source, span.end).1)
        };

        eprintln!(
            "\t{}",
            format!(
                "--> {} {}:{}{}",
                span.get_filename(compiler).display(),
                line,
                column,
                span_end
            )
            .bright_green()
            .bold()
        );
        if let Some(ctx) = context {
            if ctx.1.end <= span.start {
                self.print_context(source, ctx);
            }
        }
        eprintln!("{}", "  |".purple().bold());

        if let DiagnosticKind::Hint(addition) = &self.kind {
            eprintln!(
                "{}  {}{}",
                format!("{} |", line).purple().bold(),
                line_content,
                addition.bright_green().bold()
            );
            eprintln!(
                "  {}  {}",
                "|".purple().bold(),
                format!(
                    "{:>width$}{} {}",
                    "",
                    format!("{:+<width$}", "", width = addition.len()),
                    self.secondary_msg,
                    width = column - 1,
                )
                .bright_green()
                .bold()
            );
        } else {
            eprintln!(
                "{}  {}",
                format!("{} |", line).purple().bold(),
                line_content
            );
            eprintln!(
                "{}",
                format!(
                    "  |  {:>width$}{} {}",
                    "",
                    format!("{:^<width$}", "^", width = span.len()),
                    self.secondary_msg,
                    width = column - 1,
                )
                .purple()
                .bold()
            );
        }
        if let Some(ctx) = context {
            if ctx.1.start > span.end {
                self.print_context(source, ctx);
            }
        }
    }
}

#[derive(Debug)]
pub enum CompilerError {
    UnknownChar(char),

    // Brackets errors.
    ExtraParen,
    ExtraCurly,
    ExtraBoxed,
    UnexpectedParen(char),
    UnexpectedCurly(char),
    UnexpectedBoxed(char),
    UnmatchedParen,
    UnmatchedCurly,
    UnmatchedBoxed,

    // char/String errors.
    UnmatchedSingleQuote,
    UnknownEsc(char),
    InvalidHexEsc,
    IncompleteHexEsc,
    ExpectedLCurlyAfterUniEsc,
    InvalidUniCodePoint,
    InvalidUniEsc,

    UnexpectedEOF,
}

impl CompilerError {
    pub fn primary_msg(&self) -> String {
        match self {
            Self::UnknownChar(found) => {
                format!("Unknown character used: `{}`", found)
            }

            Self::ExtraParen => {
                format!("Extra `)` used.")
            }
            Self::ExtraCurly => {
                format!("Extra `}}` used.")
            }
            Self::ExtraBoxed => {
                format!("Extra `]` used.")
            }
            Self::UnexpectedParen(expected) => {
                format!("Expected `{}`, Found `)`.", expected)
            }
            Self::UnexpectedCurly(expected) => {
                format!("Expected `{}`, Found `}}`.", expected)
            }
            Self::UnexpectedBoxed(expected) => {
                format!("Expected `{}`, Found `]`.", expected)
            }
            Self::UnmatchedParen => {
                format!("Unmatched parentheses.")
            }
            Self::UnmatchedCurly => {
                format!("Unmatched curly bracket.")
            }
            Self::UnmatchedBoxed => {
                format!("Unmatched squared bracket.")
            }

            Self::UnmatchedSingleQuote => {
                format!("Unmatched Single Quote.")
            }
            Self::UnknownEsc(escape) => {
                format!("Unknown Escape: `\\{}`", escape)
            }
            Self::InvalidHexEsc => {
                format!("Invalid hex escape.")
            }
            Self::IncompleteHexEsc => {
                format!("Incomplete hex escape.")
            }
            Self::ExpectedLCurlyAfterUniEsc => {
                format!("Expected `{{` after \\u.")
            }
            Self::InvalidUniCodePoint => {
                format!("Invalid Unicode code point.")
            }
            Self::InvalidUniEsc => {
                format!("Invalid Unicode escape.")
            }

            Self::UnexpectedEOF => {
                format!("Unexpected end of file.")
            }
        }
    }

    pub fn secondary_msg(&self) -> String {
        match self {
            Self::UnknownChar(found) => {
                format!("Remove `{}`.", found)
            }

            Self::ExtraParen => {
                format!("Remove the extra parentheses.")
            }
            Self::ExtraCurly => {
                format!("Remove the extra curly bracket.")
            }
            Self::ExtraBoxed => {
                format!("Remove the extra squared bracket.")
            }
            Self::UnexpectedParen(expected) => {
                format!("Replace `)` with `{}`.", expected)
            }
            Self::UnexpectedCurly(expected) => {
                format!("Replace `}}` with `{}`.", expected)
            }
            Self::UnexpectedBoxed(expected) => {
                format!("Replace `]` with `{}`.", expected)
            }
            Self::UnmatchedParen => {
                format!("Match the parentheses somewhere.")
            }
            Self::UnmatchedCurly => {
                format!("Match the curly bracket somewhere.")
            }
            Self::UnmatchedBoxed => {
                format!("Match the squared bracket somewhere.")
            }

            Self::UnmatchedSingleQuote => {
                format!("Add a single quote here.")
            }

            _ => String::new(),
        }
    }
}

pub fn error(err_kind: CompilerError, span: Span) -> Diagnostic {
    Diagnostic::new(DiagnosticKind::Error, span)
        .with_primary_msg(err_kind.primary_msg())
        .with_secondary_msg(err_kind.secondary_msg())
}

pub fn error_with_context(
    err_kind: CompilerError,
    context: (String, Span),
    span: Span,
) -> Diagnostic {
    Diagnostic::new(DiagnosticKind::Error, span)
        .with_primary_msg(err_kind.primary_msg())
        .with_secondary_msg(err_kind.secondary_msg())
        .with_context(context.0, context.1)
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
        if !self.has_error() {
            return;
        }
        for diagnostic in &self.diagnostics {
            diagnostic.print(compiler);
        }
        if self.error == 1 && self.has_error() {
            eprintln!(
                "{}",
                format!("{} error has been emitted.", self.error)
                    .bright_white()
                    .bold()
            )
        } else {
            eprintln!(
                "{}",
                format!("{} errors have been emitted.", self.error)
                    .bright_white()
                    .bold()
            )
        }
    }
}
