use std::path::PathBuf;

use clap::Parser;
use logos::Logos;

#[derive(Parser)]
struct Opt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Tokenise { input: PathBuf },
    Parse { input: PathBuf },
    Assemble { input: PathBuf },
}

fn main() {
    let opt = Opt::parse();

    match opt.command {
        Command::Tokenise { input } => {
            let content = std::fs::read_to_string(input).expect("couldn't read file");
            println!("{:?}", tokenize(&content));
        }
        Command::Parse { input } => {
            let content = std::fs::read_to_string(input).expect("couldn't read file");
            let mut tokens = tokenize(&content).into_iter();
            let program = oshug_assembler::parse::Ast::consume_token_stream(&mut tokens);
            println!("{:?}", program);
        }
        Command::Assemble { input: _ } => todo!("assembler not implemented yet"),
    }
}

fn tokenize(input: &str) -> Vec<oshug_assembler::lex::Token> {
    oshug_assembler::lex::Token::lexer(input)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse error")
}
