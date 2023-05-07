use crate::assembler::program;
use crate::vm::VM;
use nom::types::CompleteStr;
use std::fmt::UpperHex;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

#[derive(Default)]
pub struct REPL {
    vm: VM,
    command_buffer: Vec<String>,
}

impl REPL {
    /// Starts interactive REPL session
    pub fn run(&mut self) {
        // buffer for user command
        let mut buffer = String::new();
        loop {
            // print and flush, since print doesn't by default
            print!(">>> ");
            io::stdout().flush().expect("Couldn't flush stdout");

            // reset buffer, read from stdin, trim trailing spaces, and add to history
            buffer.clear();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Couldn't read from stdin");
            let command = buffer.trim();
            self.command_buffer.push(command.to_string());

            match command {
                ".quit" | ".exit" => {
                    // quits
                    println!("quitting");
                    break;
                }
                ".history" => {
                    // dumps history
                    for history in &self.command_buffer {
                        println!("{history}");
                    }
                }
                ".program" => {
                    // dumps VMs program bytecode
                    pretty_print_hex(&self.vm.program, 2);
                }
                ".registers" => {
                    // dumps VMs registers + equality flag
                    pretty_print_hex(&self.vm.registers, 8);
                    println!("Equality register: {}", self.vm.equality_flag);
                }
                ".reset" => {
                    // resets VM to default state
                    self.vm = VM::default();
                }
                ".run" => {
                    // runs VM until completion
                    self.vm.run();
                }
                ".runonce" => {
                    // runs VM once
                    self.vm.run_once();
                }
                _ => {
                    // tries and parses input, pushes to program, and executes once
                    let parsed_program = program(CompleteStr(command));
                    let bytecode = match parsed_program.is_ok() {
                        true => {
                            // if parses as a valid program, treat it as assembly
                            let (_, result) = parsed_program.unwrap();
                            let bytecode = result.to_bytes();
                            match bytecode {
                                None => {
                                    println!("invalid command");
                                    continue;
                                }
                                Some(bytes) => bytes,
                            }
                        }
                        false => {
                            // otherwise treat as hex
                            match parse_hex(command) {
                                Ok(bytes) => bytes,
                                Err(_) => {
                                    println!("invalid command");
                                    continue;
                                }
                            }
                        }
                    };

                    self.vm.program.extend_from_slice(&bytecode);
                    self.vm.run_once();
                }
            }
        }
    }
}

/// Pretty prints array of types that can be represented in hex
/// Size is how much to pad each hex value
fn pretty_print_hex<T: UpperHex>(bytes: &[T], size: usize) {
    let byte_chunks = bytes.chunks(4).collect::<Vec<_>>();

    for line in byte_chunks.chunks(2) {
        for chunk in line {
            for byte in *chunk {
                print!("{byte:00$X} ", size);
            }
            print!("\t");
        }
        println!();
    }

    io::stdout().flush().expect("Couldn't flush stdout");
}

/// Parses a hex string into a list of bytes, such as "00 01 03 E8"
fn parse_hex(string: &str) -> Result<Vec<u8>, ParseIntError> {
    string
        .split(' ')
        .map(|hex_string| u8::from_str_radix(hex_string, 16))
        .collect()
}
