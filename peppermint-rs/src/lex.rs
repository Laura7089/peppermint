use super::{
    error::{Error, Span},
    Address, Literal,
};

use logos::Logos;
use num_traits::Num;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum LexError {
    InvalidInt,
    #[default]
    InvalidToken,
    UnknownInst,
}

/// One [lexical token](https://en.wikipedia.org/wiki/Lexical_token#Lexical_token_and_lexical_tokenization) in Peppermint.
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"[;#][^\n]*")] // skip comments
#[logos(error = LexError)]
pub(crate) enum Token {
    /// Instruction opcode.
    #[regex(
        r"[A-Za-z]+",
        // use the strum::FromStr implementation
        |lex| lex.slice().parse().map_err(|_| LexError::UnknownInst)
    )]
    Instruction(InstructionKind),
    /// Address literal.
    #[regex(r"\[[0-9]+\]", |lex| parse_int(debracket(lex.slice()), 10))]
    #[regex(r"\[0x[0-9a-zA-Z]+\]", |lex| parse_int(debracket(lex.slice()), 16))]
    #[regex(r"\[0b[01]+\]", |lex| parse_int(debracket(lex.slice()), 2))]
    Address(Address),
    /// Integer literal.
    #[regex(r"[0-9]+", |lex| parse_int(lex.slice(), 10))]
    #[regex(r"0x[0-9a-zA-Z]+", |lex| parse_int(lex.slice(), 16))]
    #[regex(r"0b[01]+", |lex| parse_int(lex.slice(), 2))]
    Literal(Literal),
    /// Target label for a jump instruction.
    #[regex(r":[a-zA-Z][a-zA-Z_\-0-9]*", |lex| lex.slice()[1..].to_string())]
    JumpLabel(String),
    /// Label.
    #[regex(r"[a-zA-Z][a-zA-Z_\-0-9]*:", |lex| {
        let slice = lex.slice();
        // remove the ":"
        slice[0..(slice.len() - 1)].to_string()
    })]
    Label(String),
}

fn debracket(input: &str) -> &str {
    &input[1..(input.len() - 1)]
}

// only reuturns `ErrorKind` because the lexer can attach the span for us later
fn parse_int<I: Num>(raw: &str, radix: u32) -> Result<I, LexError> {
    let raw = match radix {
        2 | 16 => &raw[2..],
        _ => raw,
    };

    I::from_str_radix(raw, radix).map_err(|_| LexError::InvalidInt)
}

/// Tokenise a source code string.
pub(crate) fn tokenise(input: &str) -> Result<Vec<(Token, Span)>, Error> {
    Token::lexer(input)
        .spanned()
        .map(|(res, span)| match res {
            Ok(tok) => Ok((tok, span)),
            Err(LexError::InvalidInt) => Err(Error::MalformedInteger { token: span }),
            Err(LexError::InvalidToken) => Err(Error::InvalidToken { token: span }),
            Err(LexError::UnknownInst) => Err(Error::UnknownInstruction { token: span }),
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

    use InstructionKind::*;
    use Token::*;

    #[test_case("load" => Instruction(Load))]
    #[test_case("lOaD" => Instruction(Load); "mixed case instr")]
    #[test_case("[0x10]" => Address(16))]
    #[test_case("[12]" => Address(12))]
    #[test_case("[0b10]" => Address(2))]
    #[test_case("0x1A" => Literal(26))]
    #[test_case("0x2a" => Literal(42))]
    #[test_case("0b1110" => Literal(14))]
    #[test_case("1120" => Literal(1120))]
    #[test_case("my10th-label:" => Label("my10th-label".to_owned()); "label")]
    #[test_case("my10th-LABEL:" => Label("my10th-LABEL".to_owned()); "label case sensitive")]
    fn single_token_lex(input: &str) -> Token {
        let mut lexer = Token::lexer(input);
        lexer.next().expect("no output").expect("lexing error")
    }

    // NOTE: these do not have to be valid code, just valid token streams
    #[test_case("LOAD 10 [0x10]label:" => vec![
        Instruction(Load),
        Literal(10),
        Address(16),
        Label("label".to_string()),
    ] ; "random sequence")]
    #[test_case("; a comment\nLOAD 10" => vec![
        Instruction(Load), Literal(10),
    ]; "comment followed by code")]
    #[test_case("; a comment with the load instruction in it\nLOAD 10" => vec![
        Instruction(Load), Literal(10),
    ]; "comment w/ instr followed by code")]
    fn token_seq(input: &str) -> Vec<Token> {
        Token::lexer(input)
            .map(|r| r.expect("lexing error"))
            .collect()
    }

    #[test_case("0x1bababab" => LexError::InvalidInt)]
    #[test_case("0b1a" => LexError::InvalidInt)]
    #[test_case("10ab" => LexError::InvalidInt)]
    #[test_case("aslkdajns" => LexError::UnknownInst)]
    fn error(input: &str) -> LexError {
        let mut lexer = Token::lexer(input);
        lexer
            .next()
            .expect("end of input")
            .err()
            .expect("lexer didn't throw an error")
    }
}
