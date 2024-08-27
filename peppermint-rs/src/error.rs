//! Parsing error reporting.

use std::{fmt::Debug, ops::Range};

/// Span of an error context in source code.
pub type Span = Range<usize>;

/// Kinds of operand instructions can expect.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::Display)]
pub enum OperandType {
    /// Memory address, like `[0x10]`.
    Address,
    /// Label for a jump instruction, like `:my-label`.
    Label,
}

/// Error in malformed input.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum Error<S: Debug = Span> {
    /// Encountered unrecognised token.
    #[error("invalid token at {token:?}")]
    InvalidToken {
        /// Span of invalid token.
        token: S,
    },
    /// Wrong kind of token for this context.
    #[error("unexpected token type at {token:?}")]
    UnexpectedToken {
        /// Span of unexpected token.
        token: S,
    },
    /// Unknown instruction opcode.
    #[error("unknown instruction type at {token:?}")]
    UnknownInstruction {
        /// Span of unknown instruction
        token: S,
    },
    /// Invalid integer literal.
    #[error("malformed integer at {token:?}")]
    MalformedInteger {
        /// Span of malformed literal
        token: S,
    },
    /// EOF encountered unexpectedly.
    #[error("EOF encountered unexpected, last token is at {last_token:?}")]
    EndOfFile {
        /// Span of the last token encoutered before EOF.
        last_token: S,
    },
    #[error("Bad operand at {operand:?} for opcode at {opcode:?}, expected a {wanted}")]
    /// Wrong kind of operand for this context.
    BadOperand {
        /// Span of the instruction expecting an operand.
        opcode: S,
        /// Span of the malformed operand.
        operand: S,
        /// Operand type the instruction wanted.
        wanted: OperandType,
    },
    /// Non-unique label value in source code.
    #[error("label defined twice in file\nfirst occurrence: {prev:?}\nsecond occurence: {this:?}")]
    DuplicateLabel {
        /// Span of the first time this label appeared.
        prev: S,
        /// Span of the next time (this time) this label appeared.
        this: S,
    },
}

impl Error<Span> {
    /// Given a `source`, convert the numeric spans in the error to string slices.
    ///
    /// Use this to increase readability of error messages when they are to be returned to the user:
    /// ```rust
    /// use peppermint::Program;
    ///
    /// let source = "LOAD [0x10]";
    /// let my_prog = Program::parse_source(source)
    ///     .map_err(|e| e.spans_to_source(source))
    ///     .expect("parse error");
    /// ```
    pub fn spans_to_source(self, source: &str) -> Error<&str> {
        match self {
            Self::InvalidToken { token: span } => Error::InvalidToken {
                token: get_span(span, source),
            },
            Self::UnexpectedToken { token: span } => Error::InvalidToken {
                token: get_span(span, source),
            },
            Self::UnknownInstruction { token: span } => Error::UnknownInstruction {
                token: get_span(span, source),
            },
            Self::MalformedInteger { token: span } => Error::MalformedInteger {
                token: get_span(span, source),
            },
            Self::EndOfFile { last_token } => Error::EndOfFile {
                last_token: get_span(last_token, source),
            },
            Self::BadOperand {
                opcode: opcode_span,
                operand: operand_span,
                wanted: wanted_operand,
            } => Error::BadOperand {
                opcode: get_span(opcode_span, source),
                operand: get_span(operand_span, source),
                wanted: wanted_operand,
            },
            Self::DuplicateLabel {
                prev: prev_span,
                this: this_span,
            } => Error::DuplicateLabel {
                prev: get_span(prev_span, source),
                this: get_span(this_span, source),
            },
        }
    }
}

/// Get the section of the source that the error refers to.
fn get_span<'a>(span: Span, source: &'a str) -> &'a str {
    &source[span]
}
