mod repl;

use assembler::Assembler;
use clap::{Parser, Subcommand};
use repl::REPL;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use vm::VM;

const PIE_HEADER_PREFIX: [u8; 4] = *b"EPIE";
const PIE_HEADER_LENGTH: usize = 64;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Repl {
        path: Option<PathBuf>,
    },
    Run {
        path: PathBuf,
        #[arg(short = 'p', long)]
        print_program: bool,
        #[arg(short = 'r', long)]
        print_registers: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Repl { path } => {
            let mut repl = REPL::default();

            if let Some(path) = path {
                // read data
                let mut file = File::open(path)?;
                let mut data = String::new();
                file.read_to_string(&mut data)?;

                // construct vm and set memory to assembled program
                let mut vm = VM::default();
                vm.program = Assembler::default().assemble(&data)?;

                repl.set_vm(vm);
            }

            repl.run();
        }
        Command::Run {
            path,
            print_program,
            print_registers,
        } => {
            // read data
            let mut file = File::open(path)?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;

            // construct and run vm
            let mut vm = VM::default();
            vm.program = Assembler::default().assemble(&data)?;
            vm.run();

            // then dump program/registers
            if print_program {
                println!("\nfinal program:");
                repl::pretty_print_hex(&vm.program, 2);
            }

            if print_registers {
                println!("\nfinal registers:");
                repl::pretty_print_hex(&vm.registers, 8);
                println!("Equality register: {}", vm.equality_flag);
            }
        }
    }

    Ok(())
}
