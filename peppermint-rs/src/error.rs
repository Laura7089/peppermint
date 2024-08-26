//! Parsing error reporting.

use std::ops::Range;
use thiserror::Error;

/// Nature of error in malformed input.
#[derive(Debug, PartialEq, Default, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum ErrorKind {
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
#[derive(Debug, Clone, Error)]
#[error("parse error of kind {kind:?} at input span {span:?}")]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Get the section of the source that the error refers to.
    pub fn get_span<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.clone()]
    }
}
