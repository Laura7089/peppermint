use logos::Logos;

fn main() {
    let in_file = std::env::args().nth(1).expect("need an input file");
    let content = std::fs::read_to_string(in_file).expect("couldn't read file");

    let tokens = oshug_assembler::lex::Token::lexer(&content)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse error");

    let mut tokens_iter = tokens.into_iter();
    let program = oshug_assembler::LabelledStatement::consume_token_stream(&mut tokens_iter);

    println!("{program:?}");
}
