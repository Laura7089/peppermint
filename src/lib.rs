// TODO: good error handling

use thiserror::Error;

pub type DoubleWord = u16;
pub type Word = u8;
pub type Address = u8; // TODO: change me to u7
pub type Literal = u16; // TODO: change me to u15

#[derive(Debug, Error, PartialEq, Default, Clone)]
pub enum ParseError {
    #[error("invalid token")]
    #[default]
    InvalidToken,
    #[error("duplicate label in source code")]
    DuplicateLabel,
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("expected token")]
    UnexpectedToken,
    #[error("wrong operand type")]
    BadOperand,
}

pub type Result<T> = std::result::Result<T, ParseError>;

// Tokenisation and lexing.
pub mod lex {
    use super::*;

    use logos::Logos;

    pub type Lexer<'a> = logos::Lexer<'a, Token>;

    pub fn tokenize(input: &str) -> Result<Vec<Token>> {
        Token::lexer(input).collect::<Result<Vec<_>>>()
    }

    #[derive(Logos, Debug, Clone, PartialEq)]
    #[logos(skip r"[ \t\n\f]+")]
    #[logos(error = ParseError)]
    pub enum Token {
        #[regex(r"[A-Za-z]+", |lex| lex.slice().parse().ok())]
        Instruction(InstructionKind),
        #[regex(r"[;#][^\n]+")]
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
        Literal(Literal),
        #[regex(r":[a-zA-Z][a-zA-Z_\-0-9]*", |lex| lex.slice()[1..].to_string())]
        JumpLabel(String),
        #[regex(r"[a-zA-Z][a-zA-Z_\-0-9]*:", |lex| {
            let slice = lex.slice();
            slice[0..(slice.len() - 1)].to_string()
        })]
        Label(String),
    }

    #[derive(Debug, Clone, strum::EnumString, PartialEq, Eq)]
    #[strum(ascii_case_insensitive)]
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

#[derive(Debug)]
pub enum Statement<L> {
    InstrLine(Instruction<L>),
    Literal(Literal),
}

#[derive(Debug)]
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

// AST construction.
pub mod parse {
    use super::*;
    use lex::Token;

    #[derive(Debug)]
    pub struct Ast {
        pub statements: Vec<LabelledStatement>,
    }

    impl Ast {
        pub fn consume_token_stream(stream: &mut impl Iterator<Item = Token>) -> Result<Self> {
            let mut statements = Vec::new();
            while let Some(stat) = LabelledStatement::take_from_token_stream(stream)? {
                statements.push(stat);
            }
            Ok(Self { statements })
        }
    }

    #[derive(Debug)]
    pub struct LabelledStatement {
        pub label: Option<String>,
        pub statement: Statement<String>,
    }

    impl LabelledStatement {
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
    use std::collections::HashMap;

    use super::*;

    type LineNum = usize;

    #[derive(Debug)]
    pub struct AstFinal {
        statements: Vec<Statement<LineNum>>,
    }

    impl AstFinal {
        pub fn sequence(&self) -> &[Statement<LineNum>] {
            &self.statements
        }

        pub fn from_ast(ast: parse::Ast) -> Result<Self> {
            // mapping from label names -> line numbers
            let line_labels = {
                let mut labels: HashMap<String, usize> = HashMap::new();
                let statement_labels = ast
                    .statements
                    .iter()
                    .enumerate()
                    .flat_map(|(i, stat)| stat.label.as_ref().map(|label| (i, label.clone())));
                for (i, label) in statement_labels {
                    if !labels.contains_key(&label) {
                        labels.insert(label, i);
                    } else {
                        return Err(ParseError::DuplicateLabel);
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

pub fn parse_final(input: &str) -> Result<flattened::AstFinal> {
    let mut tokens = lex::tokenize(&input)?.into_iter();
    let program = parse::Ast::consume_token_stream(&mut tokens)?;
    flattened::AstFinal::from_ast(program)
}
