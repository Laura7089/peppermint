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
    /// Simulate a file.
    Simulate {
        /// Addresses to read from the memory at the end of execution.
        addresses: Vec<peppermint::Address>,
        /// Number of memory words to allocate
        #[arg(short, long, default_value_t = 128_000)]
        memory_size: usize,
    },
}

fn main() {
    let opt = Opt::parse();
    let content = get_file_content(&opt.file);
    let program = peppermint::Program::parse_source(&content)
        .map_err(|e| e.spans_to_source(&content).to_string())
        .unwrap();

    match opt.command {
        Command::Parse {} => {
            println!("{:?}", program);
        }
        Command::Simulate {
            addresses,
            memory_size,
        } => {
            let mut machine = peppermint_simulate::TickTalk::new(&program, memory_size);

            machine.run_to_completion().expect("simulation error");
            for addr in addresses {
                println!("addr [0x{addr:x}]: 0x{:x}", machine.memory[addr as usize]);
            }
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
