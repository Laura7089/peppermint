use super::{
    error::{Error, ErrorKind, Span},
    Address, Literal,
};

use logos::Logos;
use num_traits::Num;

/// One [lexical token](https://en.wikipedia.org/wiki/Lexical_token#Lexical_token_and_lexical_tokenization) in Peppermint.
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = ErrorKind)]
pub(crate) enum Token {
    /// Instruction opcode.
    #[regex(r"[A-Za-z]+", |lex| lex.slice().parse().map_err(|_| ErrorKind::UnknownInstruction))]
    Instruction(InstructionKind),
    /// Code comment.
    ///
    /// Ignored for most purposes.
    #[regex(r"[;#][^\n]+", logos::skip)]
    Comment,
    /// Address literal.
    #[regex(r"\[[0-9]+\]", |lex| parse_int::<Address>(debracket(lex.slice()), 10))]
    #[regex(r"\[0x[0-9a-zA-Z]+\]", |lex| parse_int::<Address>(debracket(lex.slice()), 16))]
    #[regex(r"\[0b[01]+\]", |lex| parse_int::<Address>(debracket(lex.slice()), 2))]
    Address(Address),
    /// Integer literal.
    #[regex(r"[0-9]+", |lex| parse_int::<Literal>(lex.slice(), 10))]
    #[regex(r"0x[0-9a-zA-Z]+", |lex| parse_int::<Literal>(lex.slice(), 16))]
    #[regex(r"0b[01]+", |lex| parse_int::<Literal>(lex.slice(), 2))]
    Literal(Literal),
    /// Target label for a jump instruction.
    #[regex(r":[a-zA-Z][a-zA-Z_\-0-9]*", |lex| lex.slice()[1..].to_string())]
    JumpLabel(String),
    /// Label.
    #[regex(r"[a-zA-Z][a-zA-Z_\-0-9]*:", |lex| {
            let slice = lex.slice();
            slice[0..(slice.len() - 1)].to_string()
        })]
    Label(String),
}

fn debracket(input: &str) -> &str {
    &input[1..(input.len() - 1)]
}

fn parse_int<I: Num>(raw: &str, radix: u32) -> Result<I, ErrorKind> {
    let raw = match radix {
        2 | 16 => &raw[2..],
        _ => raw,
    };

    I::from_str_radix(raw, radix).map_err(|_| ErrorKind::MalformedInteger)
}

/// Tokenise a source code string.
pub(crate) fn tokenise(input: &str) -> Result<Vec<(Token, Span)>, Error> {
    Token::lexer(input)
        .spanned()
        .map(|(res, span)| {
            let span_again = span.clone();
            res.map(|tok| (tok, span))
                .map_err(|kind| Error::new(kind, span_again))
        })
        .collect()
}

/// Kind of instruction opcode.
#[derive(Debug, Clone, strum::EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
#[allow(missing_docs)]
pub(crate) enum InstructionKind {
    Load,
    And,
    Xor,
    Or,
    Add,
    Sub,
    Store,
    Jump,
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
    fn test_single_token_lex(input: &str) -> Token {
        let mut lexer = Token::lexer(input);
        lexer.next().expect("no output").expect("parse error")
    }
}
