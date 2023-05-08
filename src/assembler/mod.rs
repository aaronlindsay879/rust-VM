//! BYTECODE FORMAT
//! <EPIE magic number>     00 00 00 00
//! <data section offset>  <data section length>
//! <code section offset>  <code section length>

mod parser;

use crate::assembler::instruction_parser::AssemblerInstruction;
use crate::assembler::program_parsers::Program;
use crate::assembler::symbols::{Symbol, SymbolTable, SymbolType};
use crate::opcode::Opcode;
use crate::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX};
pub use program_parsers::program;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    symbols: SymbolTable,
    data: Vec<u8>,
    bytecode: Vec<u8>,
    data_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            data: vec![],
            bytecode: vec![],
            data_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                // process first phase and check for errors
                self.process_first_phase(&program);
                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }

                // check both sections exist
                if self.sections.len() != 2 {
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                // fit data to multiple of 4
                let data_len = ((self.data.len() as u32 + 3) / 4) * 4;
                self.data.resize(data_len as usize, 0);

                self.process_second_phase(&program);

                // write header and then data and bytecode
                let mut assembled_program = self.write_pie_header();

                assembled_program.extend_from_slice(&self.data);
                assembled_program.extend_from_slice(&self.bytecode);

                Ok(assembled_program)
            }
            Err(e) => Err(vec![AssemblerError::ParseError {
                error: e.to_string(),
            }]),
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        for inst in &p.instructions {
            if inst.is_label() {
                // make sure label is in a section
                if self.current_section.is_some() {
                    self.process_label_declaration(inst);
                } else {
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound {
                        instruction: self.current_instruction,
                    });
                }
            }

            if inst.is_directive() {
                self.process_directive(inst);
            }

            self.current_instruction += 1;
        }

        self.phase = AssemblerPhase::Second;
    }

    fn process_label_declaration(&mut self, inst: &AssemblerInstruction) {
        // extract label name
        let name = match inst.get_label_name() {
            None => {
                self.errors
                    .push(AssemblerError::StringConstantDeclaredWithoutLabel {
                        instruction: self.current_instruction,
                    });
                return;
            }
            Some(name) => name,
        };

        // check if label already in use
        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        let symbol = Symbol::new(name, SymbolType::Label);
        self.symbols.add_symbol(symbol);
    }

    fn process_directive(&mut self, inst: &AssemblerInstruction) {
        // extract name
        let directive_name = match inst.get_directive_name() {
            Some(name) => name,
            None => {
                println!("Directive has an invalid name: {:?}", inst);
                return;
            }
        };

        // check for and handle operands
        if inst.has_operands() {
            match &directive_name[..] {
                "asciiz" => self.handle_asciiz(inst),
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound {
                        directive: directive_name.clone(),
                    });
                    return;
                }
            }
        } else {
            // if no operands, we know its a section header
            self.process_section_header(&directive_name);
        }
    }

    fn handle_asciiz(&mut self, inst: &AssemblerInstruction) {
        // only meaningful in first pass
        if self.phase != AssemblerPhase::First {
            return;
        }

        match inst.get_string_constant() {
            None => {
                println!("String constant following an .asciiz was empty");
            }
            Some(s) => {
                match inst.get_label_name() {
                    None => {
                        // something like .asciiz 'Hello'
                        println!("Found a string constant with no associated label!");
                        return;
                    }
                    Some(name) => {
                        self.symbols
                            .set_symbol_offset(&name, self.data_offset + PIE_HEADER_LENGTH as u32);
                    }
                };

                let bytes = s.as_bytes();

                // write bytes + null byte
                self.data.extend_from_slice(bytes);
                self.data.push(0);

                self.data_offset += (bytes.len() + 1) as u32;
            }
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();

        if new_section == AssemblerSection::Unknown {
            println!(
                "Found an section header that is unknown: {:#?}",
                header_name
            );
            return;
        }

        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn process_second_phase(&mut self, program: &Program) {
        // restart instruction counting
        self.current_instruction = 0;

        for inst in &program.instructions {
            if inst.is_opcode() {
                // by second phase, all data is written - so offset symbols by
                // their position in bytecode + header offset + data offset
                if let Some(Token::LabelDeclaration { name }) = &inst.label {
                    self.symbols.set_symbol_offset(
                        &name,
                        self.bytecode.len() as u32
                            + PIE_HEADER_LENGTH as u32
                            + self.data.len() as u32,
                    );
                }

                self.bytecode
                    .extend_from_slice(&inst.to_bytes(&self.symbols).unwrap());
            }

            if inst.is_directive() {
                self.process_directive(inst);
            }

            self.current_instruction += 1;
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(PIE_HEADER_LENGTH);

        out.extend_from_slice(&PIE_HEADER_PREFIX);
        out.extend_from_slice(&[0, 0, 0, 0]);

        out.extend_from_slice(&64u32.to_be_bytes());
        out.extend_from_slice(&(self.data.len() as u32).to_be_bytes());

        out.extend_from_slice(&(64 + self.data.len() as u32).to_be_bytes());
        out.extend_from_slice(&(self.bytecode.len() as u32).to_be_bytes());

        // then pad to final length
        if out.len() < PIE_HEADER_LENGTH {
            out.resize(PIE_HEADER_LENGTH, 0);
        }

        out
    }
}

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: Option<u32> },
    Code { starting_instruction: Option<u32> },
    Unknown,
}

impl Default for AssemblerSection {
    fn default() -> Self {
        AssemblerSection::Unknown
    }
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> AssemblerSection {
        match name {
            "data" => AssemblerSection::Data {
                starting_instruction: None,
            },
            "code" => AssemblerSection::Code {
                starting_instruction: None,
            },
            _ => AssemblerSection::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound { instruction: u32 },
    StringConstantDeclaredWithoutLabel { instruction: u32 },
    SymbolAlreadyDeclared,
    UnknownDirectiveFound { directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError { error: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;

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
