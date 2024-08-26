//! Peppermint parsing, assembling and emulation.
// TODO: use workspace Cargo.toml to define lints
#![warn(clippy::pedantic)]
#![deny(missing_docs)]
#![allow(clippy::wildcard_imports)]

use thiserror::Error;

/// One memory word.
pub type Word = u8;
/// Two memory words.
pub type DoubleWord = u16;
/// Memory address.
pub type Address = u8; // TODO: change me to u7
/// Literal integer.
pub type Literal = u16; // TODO: change me to u15

/// Error originating from malformed input.
#[derive(Debug, Error, PartialEq, Default, Clone)]
pub enum ParseError {
    #[error("invalid token")]
    #[default]
    /// Encountered unrecognised token.
    InvalidToken,
    #[error("duplicate label in source code")]
    /// Non-unique label value in source code.
    DuplicateLabel,
    #[error("unexpected end of file")]
    /// EOF encountered unexpectedly.
    EndOfFile,
    #[error("expected token")]
    /// Wrong kind of token for this context.
    UnexpectedToken,
    #[error("wrong operand type")]
    /// Wrong kind of operand for this context.
    BadOperand,
}

type Result<T> = std::result::Result<T, ParseError>;

/// Tokenisation and lexing.
pub mod lex {
    use super::*;

    use logos::Logos;

    /// Peppermint lexer.
    pub type Lexer<'a> = logos::Lexer<'a, Token>;

    /// Fully run tokenisation on source code.
    ///
    /// Runs until EOF.
    ///
    /// # Errors
    ///
    /// Throws [`ParseError::InvalidToken`] if a malformed token is found.
    pub fn tokenize(input: &str) -> Result<Vec<Token>> {
        Token::lexer(input).collect::<Result<Vec<_>>>()
    }

    /// One [lexical token](https://en.wikipedia.org/wiki/Lexical_token#Lexical_token_and_lexical_tokenization) in Peppermint.
    #[derive(Logos, Debug, Clone, PartialEq)]
    #[logos(skip r"[ \t\n\f]+")]
    #[logos(error = ParseError)]
    pub enum Token {
        #[regex(r"[A-Za-z]+", |lex| lex.slice().parse().ok())]
        /// Instruction opcode.
        Instruction(InstructionKind),
        #[regex(r"[;#][^\n]+")]
        /// Code comment.
        ///
        /// Ignored for most purposes.
        Comment,
        #[regex(r"\[[0-9]+\]", |lex| {
            Address::from_str_radix(debracket(lex.slice()), 10).ok()
        })]
        #[regex(r"\[0x[0-9a-zA-Z]+\]", |lex| {
            Address::from_str_radix(debracket(strip_radix_prefix(lex.slice())), 16).ok()
        })]
        #[regex(r"\[0b[01]+\]", |lex| {
            Address::from_str_radix(debracket(strip_radix_prefix(lex.slice())), 2).ok()
        })]
        /// Address literal.
        Address(Address),
        #[regex(r"[0-9]+", |lex| {
            Literal::from_str_radix(lex.slice(), 10).ok()
        })]
        #[regex(r"0x[0-9a-zA-Z]+", |lex| {
            Literal::from_str_radix(strip_radix_prefix(lex.slice()), 16).ok()
        })]
        #[regex(r"0b[01]+", |lex| {
            Literal::from_str_radix(strip_radix_prefix(lex.slice()), 2).ok()
        })]
        /// Integer literal.
        Literal(Literal),
        #[regex(r":[a-zA-Z][a-zA-Z_\-0-9]*", |lex| lex.slice()[1..].to_string())]
        /// Target label for a jump instruction.
        JumpLabel(String),
        #[regex(r"[a-zA-Z][a-zA-Z_\-0-9]*:", |lex| {
            let slice = lex.slice();
            slice[0..(slice.len() - 1)].to_string()
        })]
        /// Label.
        Label(String),
    }

    #[derive(Debug, Clone, strum::EnumString, PartialEq, Eq)]
    #[strum(ascii_case_insensitive)]
    #[allow(missing_docs)]
    /// Kind of instruction opcode.
    pub enum InstructionKind {
        Load,
        And,
        Xor,
        Or,
        Add,
        Sub,
        Store,
        Jump,
    }

    fn debracket(input: &str) -> &str {
        &input[1..(input.len() - 1)]
    }

    fn strip_radix_prefix(input: &str) -> &str {
        &input[2..]
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case("load" => Token::Instruction(InstructionKind::Load))]
        #[test_case("[0x10]" => Token::Address(16))]
        #[test_case("[12]" => Token::Address(12))]
        #[test_case("[0b10]" => Token::Address(2))]
        #[test_case("0x1A" => Token::Literal(26))]
        #[test_case("0x2a" => Token::Literal(42))]
        #[test_case("0b1110" => Token::Literal(14))]
        #[test_case("1120" => Token::Literal(1120))]
        #[test_case("my10th-label:" => Token::Label("my10th-label".to_owned()))]
        fn test_single_token(input: &str) -> Token {
            let mut lexer = Token::lexer(input);
            lexer.next().expect("no output").expect("parse error")
        }
    }
}

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

/// AST construction.
pub mod parse {
    use super::*;
    use lex::Token;

    #[derive(Debug)]
    /// Parsed Abstract Syntax Tree.
    ///
    /// In peppermint this isn't really much of a tree owing to the non-recursive nature of the language's syntax.
    /// Note that the AST does not include comments.
    pub struct Ast {
        /// The sequence of statements which make up the AST.
        pub statements: Vec<LabelledStatement>,
    }

    impl Ast {
        /// Completely consume `stream`, parsing tokens into an AST (`Self`).
        ///
        /// # Errors
        ///
        /// See [`LabelledStatement::take_from_token_stream`].
        pub fn consume_token_stream(stream: &mut impl Iterator<Item = Token>) -> Result<Self> {
            let mut statements = Vec::new();
            while let Some(stat) = LabelledStatement::take_from_token_stream(stream)? {
                statements.push(stat);
            }
            Ok(Self { statements })
        }
    }

    /// Statement with an optional label.
    ///
    /// Note that label locations are not finalised here; hence, we're using `String` as the label type in [`Statement`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub struct LabelledStatement {
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
        pub fn take_from_token_stream(
            stream: &mut impl Iterator<Item = Token>,
        ) -> Result<Option<Self>> {
            // filter out comments
            // TODO: do this somewhere higher up the stack
            let mut stream = stream.filter(|t| t != &Token::Comment);
            let mut label = None;

            let statement_token = {
                let Some(next_token) = stream.next() else {
                    return Ok(None);
                };
                // get the statement token out by stripping out the potential label
                if let Token::Label(name) = next_token {
                    label = Some(name);
                    stream.next().ok_or(ParseError::EndOfFile)?
                } else {
                    next_token
                }
            };

            // check for literals
            if let Token::Literal(val) = statement_token {
                return Ok(Some(Self {
                    label,
                    statement: Statement::Literal(val),
                }));
            }

            // now we only have instructions left to handle
            let Token::Instruction(instr_type) = statement_token else {
                // if it's not an instruction token then the code is malformed
                return Err(ParseError::EndOfFile);
            };

            let Some(operand) = stream.next() else {
                return Err(ParseError::EndOfFile);
            };
            let full_inst = match (instr_type, operand) {
                (lex::InstructionKind::Load, Token::Address(addr)) => Instruction::Load(addr),
                (lex::InstructionKind::And, Token::Address(addr)) => Instruction::And(addr),
                (lex::InstructionKind::Xor, Token::Address(addr)) => Instruction::Xor(addr),
                (lex::InstructionKind::Or, Token::Address(addr)) => Instruction::Or(addr),
                (lex::InstructionKind::Add, Token::Address(addr)) => Instruction::Add(addr),
                (lex::InstructionKind::Sub, Token::Address(addr)) => Instruction::Sub(addr),
                (lex::InstructionKind::Store, Token::Address(addr)) => Instruction::Store(addr),
                (lex::InstructionKind::Jump, Token::JumpLabel(value)) => Instruction::Jump(value),
                _ => return Err(ParseError::BadOperand),
            };

            Ok(Some(Self {
                label,
                statement: Statement::InstrLine(full_inst),
            }))
        }
    }
}

/// Final program representation.
pub mod flattened {
    use std::collections::{hash_map::Entry, HashMap};

    use super::*;

    type LineNum = usize;

    /// Abstract Syntax Tree with labels finalised.
    ///
    /// Labels index into the AST statement list.
    ///
    /// To uphold the correctness of labels, this type does not allow mutation; if you want to inspect the statements then call [`Self::statements`].
    #[derive(Debug)]
    pub struct AstFinal {
        statements: Vec<Statement<LineNum>>,
    }

    impl AstFinal {
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
        pub fn from_ast(ast: parse::Ast) -> Result<Self> {
            // mapping from label names -> line numbers
            let line_labels = {
                let mut labels: HashMap<String, usize> = HashMap::new();
                let statement_labels =
                    ast.statements.iter().enumerate().filter_map(|(i, stat)| {
                        stat.label.as_ref().map(|label| (i, label.clone()))
                    });
                for (i, label) in statement_labels {
                    match labels.entry(label) {
                        ent @ Entry::Vacant(_) => {
                            ent.or_insert(i);
                        }
                        Entry::Occupied(_) => return Err(ParseError::DuplicateLabel),
                    }
                }
                labels
            };

            let statements = ast
                .statements
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
    }
}

/// Fully parse source code into final syntax tree.
///
/// # Errors
///
/// May throw any [`ParseError`] from any stage of parsing.
pub fn parse_final(input: &str) -> Result<flattened::AstFinal> {
    let mut tokens = lex::tokenize(input)?.into_iter();
    let program = parse::Ast::consume_token_stream(&mut tokens)?;
    flattened::AstFinal::from_ast(program)
}
