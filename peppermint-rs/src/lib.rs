//! Peppermint parsing and representation.
// TODO: use workspace Cargo.toml to define lints
#![warn(clippy::pedantic)]
#![deny(missing_docs)]
#![allow(clippy::wildcard_imports)]

use std::collections::{hash_map::Entry, HashMap};

mod lex;
use lex::{InstructionKind, Token};

pub mod error;
use error::{ParseError, ParseErrorKind, Span};

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
        stream: &mut impl Iterator<Item = (Token, error::Span)>,
    ) -> Result<Option<Self>, error::ParseError> {
        // filter out comments
        // TODO: do this somewhere higher up the stack
        let mut stream = stream.filter(|(t, _s)| t != &Token::Comment);
        let mut label = None;

        let statement_token = {
            let Some(next_token) = stream.next() else {
                // stream is depleted
                return Ok(None);
            };
            // get the statement token out by stripping out the potential label
            if let (Token::Label(name), span) = next_token {
                label = Some(name);
                stream
                    .next()
                    .ok_or(error::ParseError::new(ParseErrorKind::EndOfFile, span))?
            } else {
                next_token
            }
        };

        // check for literals
        if let (Token::Literal(val), _) = statement_token {
            return Ok(Some(Self {
                label,
                statement: Statement::Literal(val),
            }));
        }

        // now we only have instructions left to handle
        let (Token::Instruction(instr_type), _) = statement_token else {
            // if it's not an instruction token then the code is malformed
            return Err(ParseError::new(
                ParseErrorKind::EndOfFile,
                statement_token.1,
            ));
        };

        let Some((operand, span)) = stream.next() else {
            return Err(ParseError::new_no_span(ParseErrorKind::EndOfFile));
        };
        let full_inst = match (instr_type, operand) {
            (InstructionKind::Load, Token::Address(addr)) => Instruction::Load(addr),
            (InstructionKind::And, Token::Address(addr)) => Instruction::And(addr),
            (InstructionKind::Xor, Token::Address(addr)) => Instruction::Xor(addr),
            (InstructionKind::Or, Token::Address(addr)) => Instruction::Or(addr),
            (InstructionKind::Add, Token::Address(addr)) => Instruction::Add(addr),
            (InstructionKind::Sub, Token::Address(addr)) => Instruction::Sub(addr),
            (InstructionKind::Store, Token::Address(addr)) => Instruction::Store(addr),
            (InstructionKind::Jump, Token::JumpLabel(value)) => Instruction::Jump(value),
            _ => return Err(ParseError::new(ParseErrorKind::BadOperand, span)),
        };

        Ok(Some(Self {
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
    fn from_tokens(stream: &mut impl Iterator<Item = (Token, Span)>) -> Result<Self, ParseError> {
        let mut statements = Vec::new();
        while let Some(stat) = LabelledStatement::take_from_token_stream(stream)? {
            statements.push(stat);
        }

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
                        return Err(ParseError::new_no_span(ParseErrorKind::DuplicateLabel))
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
    pub fn parse_source(input: &str) -> Result<Self, ParseError> {
        let tokens = lex::tokenise(input)?;
        Program::from_tokens(&mut tokens.into_iter())
    }
}
