use crate::assembler::program;
use crate::repl::REPL;
use crate::vm::VM;
use nom::types::CompleteStr;

mod assembler;
mod instruction;
mod opcode;
mod repl;
mod vm;

fn main() {
    let text = "load $0 #100\nload$1 #200\nadd $0 $1 $2";
    let (_, program) = program(CompleteStr::from(text)).unwrap();
    let bytecode = program.to_bytes().unwrap();

    let mut vm = VM::default();
    vm.program = bytecode;
    vm.run();
    dbg!(vm.program);
    dbg!(vm.registers);
}
