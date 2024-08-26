//! Parsing error reporting.

use std::ops::Range;

/// Nature of error in malformed input.
#[derive(Debug, PartialEq, Default, Clone)]
pub enum ParseErrorKind {
    #[default]
    /// Encountered unrecognised token.
    InvalidToken,
    /// Invalid integer literal.
    MalformedInteger,
    /// Non-unique label value in source code.
    DuplicateLabel,
    /// EOF encountered unexpectedly.
    EndOfFile,
    /// Wrong kind of token for this context.
    UnexpectedToken,
    /// Wrong kind of operand for this context.
    BadOperand,
    /// Unknown instruction opcode.
    UnknownInstruction,
}

/// Span of an error context in source code.
pub type Span = Range<usize>;

/// Error originating from malformed input.
#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Span>,
}

impl ParseError {
    pub(crate) fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }

    pub(crate) fn new_no_span(kind: ParseErrorKind) -> Self {
        Self { kind, span: None }
    }
}
