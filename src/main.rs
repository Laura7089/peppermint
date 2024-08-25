use logos::Logos;

type Word = u16;

pub mod lex {
    use std::num::ParseIntError;

    use logos::Logos;

    use super::Word;

    #[derive(Logos, Debug, Clone, PartialEq)]
    #[logos(skip r"[ \t\n\f]+")]
    pub enum Token {
        #[regex(r"[A-Za-z]+", |lex| lex.slice().parse().ok())]
        Instruction(InstructionKind),
        #[regex(r"[;#][^\n]+")]
        Comment,
        #[regex(r"\[[0-9]+\]", |lex| decimal(debracket(lex.slice())).ok())]
        #[regex(r"\[0x[0-9]+\]", |lex| hexadecimal(debracket(lex.slice())).ok())]
        #[regex(r"\[0b[01]+\]", |lex| binary(debracket(lex.slice())).ok())]
        Address(Word),
        #[regex(r"[0-9]+", |lex| decimal(lex.slice()).ok())]
        #[regex(r"0x[0-9]+", |lex| hexadecimal(lex.slice()).ok())]
        #[regex(r"0b[01]+", |lex| binary(lex.slice()).ok())]
        Literal(Word),
        #[regex(r":[a-zA-Z][a-zA-Z_-]*")]
        Label,
        #[regex(r"[a-zA-Z][a-zA-Z_-]*:")]
        JumpLabel,
    }

    #[derive(Debug, Clone, strum::EnumString, PartialEq, Eq)]
    #[strum(ascii_case_insensitive)]
    pub enum InstructionKind {
        Load,
        And,
        Xor,
        OR,
        Add,
        Sub,
        Store,
        Jump,
    }

    fn debracket(input: &str) -> &str {
        &input[1..(input.len() - 1)]
    }

    fn decimal(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(input, 10)
    }

    fn hexadecimal(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(input, 16)
    }

    fn binary(input: &str) -> Result<Word, ParseIntError> {
        Word::from_str_radix(input, 2)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_case::test_case;

        #[test_case("load" => Token::Instruction(InstructionKind::Load); "instruction")]
        #[test_case("[0x10]" => Token::Address(16); "hex address")]
        fn test_single_token(input: &str) -> Token {
            let mut lexer = Token::lexer(input);
            lexer.next().expect("no output").expect("parse error")
        }
    }
}

fn main() {
    let in_file = std::env::args().nth(1).expect("need an input file");
    let content = std::fs::read_to_string(in_file).expect("couldn't read file");

    let tokens = lex::Token::lexer(&content)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse error");

    println!("{tokens:?}");
}
