//! BYTECODE FORMAT
//! ```norun
//! <EPIE magic number>     00 00 00 00
//! <data section offset>  <data section length>
//! <code section offset>  <code section length>
//! ```

use crate::assembler::errors::AssemblerError;
use crate::assembler::section::AssemblerSection;
use crate::assembler::symbols::{Symbol, SymbolTable, SymbolType};
use crate::parser::instruction::{AssemblerInstruction, DirectiveInstruction, OpcodeInstruction};
use crate::parser::operand::Operand;
use crate::parser::Program;
use crate::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX};
use std::io::Write;

mod errors;
mod section;
mod symbols;

/// Stores information used during assembly
#[derive(Default, Debug)]
pub struct Assembler {
    data_section: Vec<u8>,
    /// How many extra bytes are added to get to a multiple of 4
    data_section_padding: usize,
    code_section: Vec<u8>,
    symbols: SymbolTable,
    current_section: Option<AssemblerSection>,
}

impl Assembler {
    /// Assembles an assembly string into bytecode
    pub fn assemble(&mut self, data: &str) -> Result<Vec<u8>, AssemblerError> {
        let program = Program::parse(data).ok_or(AssemblerError::ParseError {
            error: "failed to parse assembly".to_string(),
        })?;

        self.first_pass(&program.instructions)?;
        self.second_pass(&program.instructions)?;

        let mut out = self.create_header();
        out.extend_from_slice(&self.data_section);
        out.extend_from_slice(&self.code_section);

        Ok(out)
    }

    /// First pass of assembler
    /// Scans for symbols and builds the symbol table
    fn first_pass(&mut self, program: &[AssemblerInstruction]) -> Result<(), AssemblerError> {
        let mut offset = 0;

        for instruction in program {
            match instruction {
                AssemblerInstruction::Opcode(OpcodeInstruction {
                    label: Option::None,
                    ..
                }) => {
                    // simplest case: instruction with no label
                    // simply move offset by size of instruction (4 bytes)
                    offset += 4;
                }
                AssemblerInstruction::Opcode(OpcodeInstruction {
                    label: Some(label), ..
                }) => {
                    // instruction with label, so first check we're in a section
                    if self.current_section.is_none() {
                        return Err(AssemblerError::NoSegmentDeclarationFound);
                    }

                    // then add the symbol, returning error if it already exists
                    if !self
                        .symbols
                        .add_symbol(label, Symbol::new(offset, SymbolType::Label))
                    {
                        return Err(AssemblerError::SymbolAlreadyDeclared);
                    }

                    // finally move offset by size of instruction (4 bytes)
                    offset += 4;
                }
                AssemblerInstruction::Directive(directive) => {
                    // no operands, so treat as section
                    if directive.operands.is_empty() {
                        self.current_section =
                            Some(AssemblerSection::from(directive.directive.as_str()));
                        continue;
                    }

                    // directive with label, so first check we're in a section
                    if self.current_section.is_none() {
                        return Err(AssemblerError::NoSegmentDeclarationFound);
                    }

                    // directive needs label
                    let label = directive
                        .label
                        .as_ref()
                        .ok_or(AssemblerError::StringConstantDeclaredWithoutLabel)?;

                    // then add the symbol, returning error if it already exists
                    if !self
                        .symbols
                        .add_symbol(label, Symbol::new(offset, SymbolType::Label))
                    {
                        return Err(AssemblerError::SymbolAlreadyDeclared);
                    }

                    // finally move offset by size of directive
                    offset += directive.size() as u32;
                }
            }
        }

        Ok(())
    }

    /// Generates data and code section from program
    fn second_pass(&mut self, program: &[AssemblerInstruction]) -> Result<(), AssemblerError> {
        for instruction in program {
            match instruction {
                AssemblerInstruction::Opcode(opcode) => {
                    // instructions are all 4 bytes
                    let mut buf = Vec::with_capacity(4);

                    // write opcode, and then up to 3 operands
                    buf.push(opcode.opcode as u8);
                    for operand in opcode.operands.iter().take(3) {
                        match operand {
                            Operand::Register(reg) => buf.push(*reg),
                            Operand::Value(value) => {
                                buf.extend_from_slice(&(*value as u16).to_be_bytes())
                            }
                            Operand::Label(label) => match self.symbols.get_symbol(label) {
                                None => return Err(AssemblerError::IncorrectOperand),
                                Some(symbol) => {
                                    let mut offset =
                                        symbol.offset as u16 + PIE_HEADER_LENGTH as u16;

                                    if self.current_section == Some(AssemblerSection::Code) {
                                        // factor in data section padding
                                        offset += self.data_section_padding as u16;
                                    }

                                    buf.extend_from_slice(&offset.to_be_bytes())
                                }
                            },
                            Operand::String(_) => return Err(AssemblerError::IncorrectOperand),
                        }
                    }

                    // then ensure instruction is padded to 4 bytes
                    if buf.len() < 4 {
                        buf.resize(4, 0);
                    }

                    // then write buffer to main bytecode
                    self.code_section.extend_from_slice(&buf);
                }
                AssemblerInstruction::Directive(directive) => {
                    // no operands, so treat as section
                    if directive.operands.is_empty() {
                        self.current_section =
                            Some(AssemblerSection::from(directive.directive.as_str()));

                        // if moved onto code section
                        if self.current_section == Some(AssemblerSection::Code) {
                            // pad data section to multiple of 4 bytes
                            let data_len = ((self.data_section.len() as u32 + 3) / 4) * 4;

                            let diff = data_len as usize - self.data_section.len();
                            self.data_section_padding = diff;
                            self.data_section.resize(data_len as usize, 0);
                        }
                        continue;
                    }

                    match &directive.directive[..] {
                        "asciiz" => {
                            let string = match directive.operands.first() {
                                Some(Operand::String(string)) => string,
                                _ => return Err(AssemblerError::IncorrectOperand),
                            };

                            match self.current_section {
                                Some(AssemblerSection::Data) => {
                                    self.data_section.extend_from_slice(string.as_bytes());
                                    self.data_section.push(0);
                                }
                                Some(AssemblerSection::Code) => {
                                    self.code_section.extend_from_slice(string.as_bytes());
                                    self.code_section.push(0);
                                }
                                _ => return Err(AssemblerError::NoSegmentDeclarationFound),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    /// Creates 64 byte header
    fn create_header(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(PIE_HEADER_LENGTH);

        out.extend_from_slice(&PIE_HEADER_PREFIX);
        out.extend_from_slice(&[0, 0, 0, 0]);

        out.extend_from_slice(&64u32.to_be_bytes());
        out.extend_from_slice(&(self.data_section.len() as u32).to_be_bytes());

        out.extend_from_slice(&(64 + self.data_section.len() as u32).to_be_bytes());
        out.extend_from_slice(&(self.code_section.len() as u32).to_be_bytes());

        // then pad to final length
        if out.len() < PIE_HEADER_LENGTH {
            out.resize(PIE_HEADER_LENGTH, 0);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    hello: .asciiz 'Hello'
                                    world: .asciiz 'world!'
                                .code
                                    inc $5
                                    loop:
                                    inc $5
                                    djmp @loop"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 16, 0, 0, 0, 80, 0, 0, 0, 12, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [
            72, 101, 108, 108, 111, 0, 119, 111, 114, 108, 100, 33, 0, 0, 0, 0,
        ];
        let expected_code = [19, 5, 0, 0, 19, 5, 0, 0, 21, 0, 84, 0];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .chain(expected_code.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }
}
