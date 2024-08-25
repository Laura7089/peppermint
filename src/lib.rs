type Word = u16;

pub mod lex {
    use std::num::ParseIntError;

    use logos::Logos;

    use super::Word;
    pub type Lexer<'a> = logos::Lexer<'a, Token>;

    #[derive(Logos, Debug, Clone, PartialEq)]
    #[logos(skip r"[ \t\n\f]+")]
    pub enum Token {
        #[regex(r"[A-Za-z]+", |lex| lex.slice().parse().ok())]
        Instruction(InstructionKind),
        #[regex(r"[;#][^\n]+")]
        Comment,
        #[regex(r"\[[0-9]+\]", |lex| decimal(debracket(lex.slice())).ok())]
        #[regex(r"\[0x[0-9a-zA-Z]+\]", |lex| hexadecimal(debracket(lex.slice())).ok())]
        #[regex(r"\[0b[01]+\]", |lex| binary(debracket(lex.slice())).ok())]
        Address(Word),
        #[regex(r"[0-9]+", |lex| decimal(lex.slice()).ok())]
        #[regex(r"0x[0-9a-zA-Z]+", |lex| hexadecimal(lex.slice()).ok())]
        #[regex(r"0b[01]+", |lex| binary(lex.slice()).ok())]
        Literal(Word),
        #[regex(r":[a-zA-Z][a-zA-Z_-]*", |lex| lex.slice()[1..].to_string())]
        Label(String),
        #[regex(r"[a-zA-Z][a-zA-Z_-]*:", |lex| {
            let slice = lex.slice();
            slice[0..(slice.len() - 1)].to_string()
        })]
        JumpLabel(String),
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

    fn decimal(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(input, 10)
    }

    fn hexadecimal(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(strip_radix_prefix(input), 16)
    }

    fn binary(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(strip_radix_prefix(input), 2)
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
        fn test_single_token(input: &str) -> Token {
            let mut lexer = Token::lexer(input);
            lexer.next().expect("no output").expect("parse error")
        }
    }
}

pub mod parse {
    use super::*;
    use lex::Token;

    #[derive(Debug)]
    pub struct Ast {
        statements: Vec<LabelledStatement>,
    }

    impl Ast {
        pub fn consume_token_stream(stream: &mut impl Iterator<Item = Token>) -> Self {
            let mut statements = Vec::new();

            while let Some(statement) = LabelledStatement::take_from_token_stream(stream) {
                statements.push(statement);
            }

            Self { statements }
        }
    }

    #[derive(Debug)]
    pub struct LabelledStatement {
        label: Option<String>,
        statement: Statement,
    }

    impl LabelledStatement {
        pub fn take_from_token_stream(stream: &mut impl Iterator<Item = Token>) -> Option<Self> {
            // filter out comments
            // TODO: do this somewhere higher up the stack
            let mut stream = stream.filter(|t| t != &Token::Comment);
            let mut label = None;

            let statement_token = {
                let next_token = stream.next()?;
                // get the statement token out by stripping out the potential label
                if let Token::Label(name) = next_token {
                    label = Some(name);
                    stream.next()?
                } else {
                    next_token
                }
            };

            // check for literals
            if let Token::Literal(val) = statement_token {
                return Some(Self {
                    label,
                    statement: Statement::Literal(val),
                });
            }

            // now we only have instructions left to handle
            let Token::Instruction(instr_type) = statement_token else {
                // if it's not an instruction token then the code is malformed
                return None;
            };

            let operand = stream.next()?;
            let full_inst = match (instr_type, operand) {
                (lex::InstructionKind::Load, Token::Address(addr)) => FullInstruction::Load(addr),
                (lex::InstructionKind::And, Token::Address(addr)) => FullInstruction::And(addr),
                (lex::InstructionKind::Xor, Token::Address(addr)) => FullInstruction::Xor(addr),
                (lex::InstructionKind::Or, Token::Address(addr)) => FullInstruction::Or(addr),
                (lex::InstructionKind::Add, Token::Address(addr)) => FullInstruction::Add(addr),
                (lex::InstructionKind::Sub, Token::Address(addr)) => FullInstruction::Sub(addr),
                (lex::InstructionKind::Store, Token::Address(addr)) => FullInstruction::Store(addr),
                (lex::InstructionKind::Jump, Token::JumpLabel(value)) => {
                    FullInstruction::Jump(value)
                }
                _ => return None,
            };

            Some(Self {
                label,
                statement: Statement::InstrLine(full_inst),
            })
        }
    }

    #[derive(Debug)]
    pub enum Statement {
        InstrLine(FullInstruction),
        Literal(Word),
    }

    #[derive(Debug)]
    pub enum FullInstruction {
        Load(Word),
        And(Word),
        Xor(Word),
        Or(Word),
        Add(Word),
        Sub(Word),
        Store(Word),
        Jump(String),
    }
}
