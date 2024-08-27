//! Peppermint parsing and representation.
// TODO: use workspace Cargo.toml to define lints
#![warn(clippy::pedantic)]
#![deny(missing_docs)]
#![allow(clippy::wildcard_imports)]

use std::collections::{hash_map::Entry, HashMap};

mod lex;
use lex::{InstructionKind, Token};

pub mod error;
use error::{Error, Span};

/// One memory word.
pub type Word = u8;
/// Two memory words.
pub type DoubleWord = u16;
/// Memory address.
pub type Address = u16;
/// Literal integer.
pub type Literal = u16;
type StatNum = usize;

/// Statement in Peppermint.
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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
            return Some(Err(Error::UnexpectedToken {
                token: first_token.1,
            }));
        };

        let Some((operand, operand_span)) = stream.next() else {
            return Some(Err(Error::EndOfFile {
                last_token: opcode_span,
            }));
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
            // if it's a jump without a label
            (Jump, Token::Address(_)) => {
                return Some(Err(Error::BadOperand {
                    opcode: opcode_span,
                    operand: operand_span,
                    wanted: error::OperandType::Label,
                }))
            }
            _ => {
                return Some(Err(Error::BadOperand {
                    opcode: opcode_span,
                    operand: operand_span,
                    wanted: error::OperandType::Address,
                }))
            }
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
        use Instruction::*;
        use Statement::*;
        let stat_stream = std::iter::from_fn(|| Statement::take_from_token_stream(stream));

        let mut statements = Vec::new();
        let mut labels: HashMap<String, (usize, Span)> = HashMap::new();
        for (i, stat) in stat_stream.enumerate() {
            let (stat, span) = stat?;
            if let Statement::Label(name) = &stat {
                // TODO: remove clone
                match labels.entry(name.clone()) {
                    ent @ Entry::Vacant(_) => {
                        ent.or_insert((i, span));
                    }
                    Entry::Occupied(entry) => {
                        let (_, prev_span) = entry.remove();
                        return Err(Error::DuplicateLabel {
                            prev: prev_span,
                            this: span,
                        });
                    }
                }
            }
            statements.push(stat);
        }

        let statements = statements
            .into_iter()
            .map(|stat| {
                // TODO: this is ridiculous
                match stat {
                    InstrLine(Jump(name)) => InstrLine(Jump(labels[&name].0)),
                    Literal(l) => Literal(l),
                    InstrLine(Load(a)) => InstrLine(Load(a)),
                    InstrLine(And(a)) => InstrLine(And(a)),
                    InstrLine(Xor(a)) => InstrLine(Xor(a)),
                    InstrLine(Or(a)) => InstrLine(Or(a)),
                    InstrLine(Add(a)) => InstrLine(Add(a)),
                    InstrLine(Sub(a)) => InstrLine(Sub(a)),
                    InstrLine(Store(a)) => InstrLine(Store(a)),
                    Label(name) => Label(name),
                }
            })
            .collect();

        Ok(Self { statements })
    }

    /// Fully parse source code into final syntax tree.
    ///
    /// # Errors
    ///
    /// May throw any [`Error`] from any stage of parsing.
    pub fn parse_source(input: &str) -> Result<Self, Error> {
        let tokens = lex::tokenise(input)?;
        Program::from_tokens(&mut tokens.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    use error::Error;
    use Instruction::*;
    use Statement::*;

    #[test_case("load [0x10]" => vec![InstrLine(Load(16))])]
    #[test_case("load [0x10] 10 jump :label" => vec![
        InstrLine(Load(16)),
        Literal(10),
        InstrLine(Jump("label".to_string()))
    ])]
    fn statement_sequence_from_str(input: &str) -> Vec<Statement<String>> {
        let mut stream = lex::tokenise(input).expect("lexer error").into_iter();
        std::iter::from_fn(|| Statement::take_from_token_stream(&mut stream))
            .map(|res| res.unwrap().0)
            .collect()
    }

    #[test_case("load :label" => matches Error::BadOperand { 
        wanted: error::OperandType::Address, 
        ..
    })]
    fn statement_error_from_str(input: &str) -> Error {
        let mut stream = lex::tokenise(input).expect("lexer error").into_iter();
        let res = std::iter::from_fn(|| Statement::take_from_token_stream(&mut stream))
            .collect::<Result<Vec<_>, _>>();

        res.err().expect("no error thrown")
    }
}
