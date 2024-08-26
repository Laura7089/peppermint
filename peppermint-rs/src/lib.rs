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
type StatNum = usize;

/// Statement in Peppermint.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Statement<L> {
    Label(String),
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

impl Statement<String> {
    /// Take the next (labelled) statement from `stream`.
    ///
    /// Mutates `stream`, leaving everything after the next (valid) statement.
    ///
    /// # Errors
    ///
    /// Can throw [`Error::EndOfFile`], [`Error::BadOperand`] or [`Error::InvalidToken]`.
    fn take_from_token_stream(
        stream: &mut impl Iterator<Item = (Token, Span)>,
    ) -> Option<Result<(Self, Span), Error>> {
        #[allow(clippy::enum_glob_use)]
        use InstructionKind::*;

        let first_token = stream.next()?;

        // handle labels and literals
        match first_token {
            (Token::Label(name), span) => return Some(Ok((Self::Label(name), span))),
            (Token::Literal(val), span) => return Some(Ok((Self::Literal(val), span))),
            _ => {}
        }

        // now we only have instructions left to handle
        let (Token::Instruction(opcode), opcode_span) = first_token else {
            // if it's not an instruction token then the code is malformed
            // we don't expect to run into comments because they're ignored by the lexer
            return Some(Err(Error::new(ErrorKind::InvalidToken, first_token.1)));
        };

        let Some((operand, operand_span)) = stream.next() else {
            return Some(Err(Error::new(ErrorKind::EndOfFile, opcode_span)));
        };
        // construct the span of the full instruction
        let whole_span = (opcode_span.start)..(operand_span.end);

        let full_inst = match (opcode, operand) {
            (Load, Token::Address(addr)) => Instruction::Load(addr),
            (And, Token::Address(addr)) => Instruction::And(addr),
            (Xor, Token::Address(addr)) => Instruction::Xor(addr),
            (Or, Token::Address(addr)) => Instruction::Or(addr),
            (Add, Token::Address(addr)) => Instruction::Add(addr),
            (Sub, Token::Address(addr)) => Instruction::Sub(addr),
            (Store, Token::Address(addr)) => Instruction::Store(addr),
            (Jump, Token::JumpLabel(value)) => Instruction::Jump(value),
            _ => return Some(Err(Error::new(ErrorKind::BadOperand, operand_span))),
        };

        Some(Ok((Self::InstrLine(full_inst), whole_span)))
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
    statements: Vec<Statement<StatNum>>,
}

impl Program {
    /// Read-only reference to internal statement list/AST.
    #[must_use]
    pub fn statements(&self) -> &[Statement<StatNum>] {
        &self.statements
    }

    /// Parse a token stream and make the labels absolute.
    fn from_tokens(stream: &mut impl Iterator<Item = (Token, Span)>) -> Result<Self, Error> {
        let stat_stream = std::iter::from_fn(|| Statement::take_from_token_stream(stream));

        let mut statements = Vec::new();
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (i, stat) in stat_stream.enumerate() {
            let (stat, span) = stat?;
            if let Statement::Label(name) = &stat {
                // TODO: remove clone
                match labels.entry(name.clone()) {
                    ent @ Entry::Vacant(_) => {
                        ent.or_insert(i);
                    }
                    Entry::Occupied(_) => return Err(Error::new(ErrorKind::DuplicateLabel, span)),
                }
            }
            statements.push(stat);
        }

        let statements = statements
            .into_iter()
            .map(|stat| {
                // TODO: this is ridiculous
                match stat {
                    Statement::InstrLine(Instruction::Jump(name)) => {
                        Statement::InstrLine(Instruction::Jump(labels[&name]))
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
                    Statement::Label(name) => Statement::Label(name),
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
