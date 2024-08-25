use std::{
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser)]
struct Opt {
    #[clap(subcommand)]
    command: Command,

    /// File to read assembly from.
    #[clap(short, long, default_value = "/dev/stdin")]
    file: PathBuf,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Tokenise the input file.
    Tokenise {},
    /// Parse the input file into an AST.
    Parse {},
    /// Assemble the input file into raw machine code.
    Assemble { output_file: PathBuf },
}

fn main() {
    let opt = Opt::parse();
    let content = get_file_content(&opt.file);

    match opt.command {
        Command::Tokenise {} => {
            println!("{:?}", oshug_assembler::lex::tokenize(&content));
        }
        Command::Parse {} => {
            let mut tokens = oshug_assembler::lex::tokenize(&content).into_iter();
            let program = oshug_assembler::parse::Ast::consume_token_stream(&mut tokens);
            println!("{:?}", program);
        }
        Command::Assemble { output_file: _ } => todo!("assembler not implemented yet"),
    }
}

fn get_file_content(input: &Path) -> String {
    if input.to_str() == Some("-") {
        let mut buf = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buf).expect("couldn't read stdin");

        buf
    } else {
        std::fs::read_to_string(input).expect("couldn't read file")
    }
}
