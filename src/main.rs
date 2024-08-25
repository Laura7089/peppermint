use logos::Logos;

fn main() {
    let in_file = std::env::args().nth(1).expect("need an input file");
    let content = std::fs::read_to_string(in_file).expect("couldn't read file");

    let tokens = oshug_assembler::lex::Token::lexer(&content)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse error");

    println!("{tokens:?}");
}
