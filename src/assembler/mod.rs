mod directive_parsers;
mod instruction_parser;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
mod program_parsers;
mod register_parsers;

use crate::assembler::instruction_parser::AssemblerInstruction;
use crate::assembler::program_parsers::Program;
use crate::opcode::Opcode;
use crate::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX};
use nom::types::CompleteStr;
pub use program_parsers::program;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();

                self.process_first_phase(&program);
                let body = self.process_second_phase(&program);
                assembled_program.extend_from_slice(&body);

                Some(assembled_program)
            }
            Err(e) => {
                println!("failed {e}");
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        p.to_bytes(&self.symbols)
            .expect("failed to convert program to bytecode")
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut position = 0;

        for inst in &p.instructions {
            if inst.is_label() {
                match inst.label_name() {
                    None => {}
                    Some(name) => {
                        let symbol = Symbol::new(name, SymbolType::Label, position);
                        self.symbols.add_symbol(symbol);
                    }
                }
            }

            position += 4;
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(PIE_HEADER_LENGTH);

        out.extend_from_slice(&PIE_HEADER_PREFIX);
        if out.len() < PIE_HEADER_LENGTH {
            out.resize(PIE_HEADER_LENGTH, 0);
        }

        out
    }
}

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert_eq!(v.is_some(), false);
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string =
            "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        let program = asm.assemble(test_string).unwrap();
        let mut vm = VM::default();
        assert_eq!(program.len(), 28 + PIE_HEADER_LENGTH);
        vm.program.extend_from_slice(&program);
        assert_eq!(vm.program.len(), 28 + PIE_HEADER_LENGTH);
    }
}
