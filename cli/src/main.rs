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
    /// Parse the input file into an AST.
    Parse {},
    /// Assemble the input file into raw machine code.
    Assemble { output_file: PathBuf },
}

fn main() {
    let opt = Opt::parse();
    let content = get_file_content(&opt.file);

    match opt.command {
        Command::Parse {} => {
            let program = peppermint::Program::parse_source(&content).expect("parse error");
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
