use crate::repl::REPL;

mod instruction;
mod opcode;
mod repl;
mod vm;

fn main() {
    let mut repl = REPL::default();
    repl.run();
}
