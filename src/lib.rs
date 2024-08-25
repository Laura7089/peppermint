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
        OR,
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
