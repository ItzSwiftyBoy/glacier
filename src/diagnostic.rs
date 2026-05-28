use crate::{
    compiler::Compiler,
    get_file_content, get_line_from_index,
    utils::{FileId, Span},
};
use colored::{ColoredString, Colorize};

#[macro_export]
macro_rules! diag {
    ( $kind:expr, $p_msg:expr, $s_msg:expr, $span:expr ) => {
        Diagnostic::new($kind, $p_msg, $s_msg, $span)
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

#[derive(Debug, PartialEq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Hint(String), // 0 = The character that needs to be added
}

impl DiagnosticKind {
    pub fn associated_color(&self, input: &str) -> ColoredString {
        match self {
            Self::Error => input.bright_red().bold(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Header {
    level: DiagnosticKind,
    msg: String,
    file_id: FileId,
    span: Span,
}

impl Header {
    pub fn new(level: DiagnosticKind, msg: String, span: Span) -> Self {
        Self {
            level,
            file_id: 0,
            msg,
            span,
        }
    }

    pub fn set_file_id(&mut self, file_id: FileId) {
        self.file_id = file_id;
    }

    pub fn print(&self, compiler: &Compiler) {
        match self.level {
            DiagnosticKind::Error => {
                println!("{}: {}", "Error".bright_red().bold(), &self.msg)
            }
            _ => unimplemented!(),
        }
        let path = compiler.get_filepath(self.file_id);
        let (line, column) = get_line_and_column(&get_file_content(&path), self.span.start);
        print!(
            "\t{} ",
            format!("--> {}", path.display())
                .custom_color((100, 128, 250))
                .bold()
        );
        if self.span.len() == 1 {
            println!("[{}:{}]", line, column);
        } else {
            println!("[{}:{}-{}]", line, column, (column + self.span.len() - 1));
        }
    }
}

#[derive(Debug)]
struct SourceWithMsg {
    file_id: FileId,
    span: Vec<Span>,
    msgs: Vec<String>,
}

impl SourceWithMsg {
    pub fn new(span: Span, msg: String) -> Self {
        Self {
            file_id: 0,
            span: vec![span],
            msgs: vec![msg],
        }
    }

    // pub fn error(&mut self, msg: impl Into<String>, span: Span, file_id: FileId) {
    // if !self.msg.is_empty() && !self.0.is_empty() {
    //     panic!("Cannot push `String` in `Vec` for the first (index 0) element of the `Vec` should be an error message.");
    // }
    // self.0.insert(file_id, (msg.into(), span));
    // }

    pub fn set_file_id(&mut self, file_id: FileId) {
        self.file_id = file_id;
    }

    pub fn add_msg(&mut self, msg: impl Into<String>, span: Span) {
        self.span.push(span);
        self.msgs.push(msg.into());
    }

    pub fn print(&self, compiler: &Compiler, level: &DiagnosticKind) {
        let file_content = get_file_content(&compiler.get_filepath(self.file_id));
        let line_content = &get_line_from_index(&file_content, self.span[0].start);
        let (line, column) = get_line_and_column(line_content, self.span[0].start);
        println!("{} {}", line.to_string().hidden(), "|".bright_purple());
        println!(
            "{} {}",
            format!("{} {}", line, "|").bright_purple(),
            line_content
        );
        println!(
            "{} {} {:>width$}{} {}",
            line.to_string().hidden(),
            "|".bright_purple(),
            "",
            level.associated_color(&format!("{:^<width$}", "^", width = self.span[0].len())),
            level.associated_color(&self.msgs[0]),
            width = column - 1,
        );
        println!("{} {}", line.to_string().hidden(), "|".bright_purple());
    }
}

#[derive(Debug, Default)]
struct Annotation {
    pub notes: Vec<String>,
    pub hints: Vec<String>,
}

impl Annotation {
    pub fn print(&self) {
        if self.notes.is_empty() && self.hints.is_empty() {
            println!();
            return;
        }
        println!("—————————————————————————");
        for note in &self.notes {
            println!("\t{} {}: {}", "=>".bright_purple(), "note".on_green(), note);
        }
        for hint in &self.hints {
            println!("\t{} {}: {}", "=>".bright_purple(), "hint".on_cyan(), hint);
        }
        println!();
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    header: Header,
    src: SourceWithMsg,
    annotation: Annotation,
}

impl Diagnostic {
    pub fn new(
        kind: DiagnosticKind,
        primary_msg: impl Into<String>,
        secondary_msg: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            header: Header::new(kind, primary_msg.into(), span),
            src: SourceWithMsg::new(span, secondary_msg.into()),
            annotation: Annotation::default(),
        }
    }

    pub fn set_file_id(&mut self, file_id: FileId) {
        self.header.set_file_id(file_id);
    }

    pub fn add_note(mut self, note: String) -> Self {
        self.annotation.notes.push(note);
        self
    }

    pub fn add_hint(mut self, hint: String) -> Self {
        self.annotation.hints.push(hint);
        self
    }

    pub fn is_err(&self) -> bool {
        if self.header.level == DiagnosticKind::Error {
            true
        } else {
            false
        }
    }

    pub fn print(&self, compiler: &Compiler) {
        self.header.print(compiler);
        self.src.print(compiler, &self.header.level);
        self.annotation.print();
    }

    /* fn print_context(&self, source: &str, context: &(String, Span)) {
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

    fn print(&self, compiler: &Compiler, file_id: FileId) {
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
        let source = &get_file_content(&compiler.get_filepath(file_id));
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
                compiler.get_filepath(file_id).display(),
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
    } */
}

/* pub fn error_with_context(
    err_kind: CompilerError,
    context: (String, Span),
    span: Span,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticKind::Error,
        0,
        err_kind.primary_msg(),
        err_kind.secondary_msg(),
        span,
    )
    .with_context(context.0, context.1)
} */

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

    pub fn add(&mut self, diagnostic: Diagnostic, file_id: FileId) {
        if diagnostic.is_err() {
            self.error += 1;
        }
        let mut diag = diagnostic;
        diag.set_file_id(file_id);
        self.diagnostics.push(diag);
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
