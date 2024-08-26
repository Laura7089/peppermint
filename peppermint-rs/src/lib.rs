//! Peppermint parsing and representation.
// TODO: use workspace Cargo.toml to define lints
#![warn(clippy::pedantic)]
#![deny(missing_docs)]
#![allow(clippy::wildcard_imports)]

use std::collections::{hash_map::Entry, HashMap};

mod lex;
use lex::{InstructionKind, Token};

pub mod error;
use error::{Error, ErrorKind, Span};

/// One memory word.
pub type Word = u8;
/// Two memory words.
pub type DoubleWord = u16;
/// Memory address.
pub type Address = u8; // TODO: change me to u7
/// Literal integer.
pub type Literal = u16; // TODO: change me to u15
type LineNum = usize;

/// Statement in Peppermint.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Statement<L> {
    InstrLine(Instruction<L>),
    Literal(Literal),
}

/// Instruction w/ opcode in Peppermint.
///
/// Generic over how the `jump` instruction refers to labels.
/// This is to reduce code duplication between parsing and finalisation steps.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Instruction<L> {
    Load(Address),
    And(Address),
    Xor(Address),
    Or(Address),
    Add(Address),
    Sub(Address),
    Store(Address),
    Jump(L),
}

/// Statement with an optional label.
///
/// Note that label locations are not finalised here; hence, we're using `String` as the label type in [`Statement`].
#[derive(Debug)]
#[allow(missing_docs)]
struct LabelledStatement {
    pub label: Option<String>,
    pub statement: Statement<String>,
}

impl LabelledStatement {
    /// Take the next (labelled) statement from `stream`.
    ///
    /// Mutates `stream`, leaving everything after the next (valid) statement.
    ///
    /// # Errors
    ///
    /// Can throw [`ParseError::EndOfFile`], [`ParseError::BadOperand`] or [`ParseError::InvalidToken]`.
    fn take_from_token_stream(
        stream: &mut impl Iterator<Item = (Token, Span)>,
    ) -> Option<Result<Self, Error>> {
        // get the statement token out by stripping out the potential label
        let mut label = None;
        let statement_token = match stream.next()? {
            (Token::Label(name), span) => {
                label = Some(name);
                let Some(tok) = stream.next() else {
                    return Some(Err(Error::new(ErrorKind::EndOfFile, span)));
                };
                tok
            }
            t => t,
        };

        let (opcode, operand, op_span) = match statement_token {
            // check for literals
            (Token::Literal(val), _) => {
                return Some(Ok(Self {
                    label,
                    statement: Statement::Literal(val),
                }))
            }
            // now we only have instructions left to handle
            // check that an operand follows it
            (Token::Instruction(instr_type), span) => match stream.next() {
                Some((operand, op_span)) => (instr_type, operand, op_span),
                None => return Some(Err(Error::new(ErrorKind::EndOfFile, span))),
            },
            // if it's not an instruction token then the code is malformed
            // we don't expect to run into comments because they're ignored by the lexer
            (_, span) => return Some(Err(Error::new(ErrorKind::InvalidToken, span))),
        };

        use InstructionKind::*;

        let full_inst = match (opcode, operand) {
            (Load, Token::Address(addr)) => Instruction::Load(addr),
            (And, Token::Address(addr)) => Instruction::And(addr),
            (Xor, Token::Address(addr)) => Instruction::Xor(addr),
            (Or, Token::Address(addr)) => Instruction::Or(addr),
            (Add, Token::Address(addr)) => Instruction::Add(addr),
            (Sub, Token::Address(addr)) => Instruction::Sub(addr),
            (Store, Token::Address(addr)) => Instruction::Store(addr),
            (Jump, Token::JumpLabel(value)) => Instruction::Jump(value),
            _ => return Some(Err(Error::new(ErrorKind::BadOperand, op_span))),
        };

        Some(Ok(Self {
            label,
            statement: Statement::InstrLine(full_inst),
        }))
    }
}

/// Parsed and checked Peppermint program.
///
/// Abstract Syntax Tree with labels finalised.
/// Labels index into the AST statement list.
///
/// To uphold the correctness of labels, this type does not allow mutation; if you want to inspect the statements then call [`Self::statements`].
#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement<LineNum>>,
}

impl Program {
    /// Read-only reference to internal statement list/AST.
    #[must_use]
    pub fn statements(&self) -> &[Statement<LineNum>] {
        &self.statements
    }

    /// Consume a non-finalised [`parse::Ast`] and make the labels absolute.
    ///
    /// # Errors
    ///
    /// Throws [`ParseError::DuplicateLabel`] if the same label is encountered twice.
    fn from_tokens(stream: &mut impl Iterator<Item = (Token, Span)>) -> Result<Self, Error> {
        let statements: Vec<_> =
            std::iter::from_fn(|| LabelledStatement::take_from_token_stream(stream))
                .collect::<Result<_, _>>()?;

        // mapping from label names -> line numbers
        let line_labels = {
            let mut labels: HashMap<String, usize> = HashMap::new();
            let statement_labels = statements
                .iter()
                .enumerate()
                .filter_map(|(i, stat)| stat.label.as_ref().map(|label| (i, label.clone())));
            for (i, label) in statement_labels {
                match labels.entry(label) {
                    ent @ Entry::Vacant(_) => {
                        ent.or_insert(i);
                    }
                    Entry::Occupied(_) => {
                        return Err(Error::new_no_span(ErrorKind::DuplicateLabel))
                    }
                }
            }
            labels
        };

        let statements = statements
            .into_iter()
            .map(|stat| {
                let stat = stat.statement;
                // TODO: this is ridiculous
                match stat {
                    Statement::InstrLine(Instruction::Jump(l)) => {
                        Statement::InstrLine(Instruction::Jump(line_labels[&l]))
                    }
                    Statement::Literal(l) => Statement::Literal(l),
                    Statement::InstrLine(Instruction::Load(a)) => {
                        Statement::InstrLine(Instruction::Load(a))
                    }
                    Statement::InstrLine(Instruction::And(a)) => {
                        Statement::InstrLine(Instruction::And(a))
                    }
                    Statement::InstrLine(Instruction::Xor(a)) => {
                        Statement::InstrLine(Instruction::Xor(a))
                    }
                    Statement::InstrLine(Instruction::Or(a)) => {
                        Statement::InstrLine(Instruction::Or(a))
                    }
                    Statement::InstrLine(Instruction::Add(a)) => {
                        Statement::InstrLine(Instruction::Add(a))
                    }
                    Statement::InstrLine(Instruction::Sub(a)) => {
                        Statement::InstrLine(Instruction::Sub(a))
                    }
                    Statement::InstrLine(Instruction::Store(a)) => {
                        Statement::InstrLine(Instruction::Store(a))
                    }
                }
            })
            .collect();

        Ok(Self { statements })
    }

    /// Fully parse source code into final syntax tree.
    ///
    /// # Errors
    ///
    /// May throw any [`ParseError`] from any stage of parsing.
    pub fn parse_source(input: &str) -> Result<Self, Error> {
        let tokens = lex::tokenise(input)?;
        Program::from_tokens(&mut tokens.into_iter())
    }
}
