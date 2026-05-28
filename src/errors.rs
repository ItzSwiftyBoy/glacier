use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    utils::Span,
};

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
    Diagnostic::new(
        DiagnosticKind::Error,
        err_kind.primary_msg(),
        err_kind.secondary_msg(),
        span,
    )
}
