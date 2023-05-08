//! BYTECODE FORMAT
//! ```norun
//! <EPIE magic number>     00 00 00 00
//! <data section offset>  <data section length>
//! <code section offset>  <code section length>
//! ```

use crate::assembler::errors::AssemblerError;
use crate::assembler::section::AssemblerSection;
use crate::assembler::symbols::{Symbol, SymbolTable, SymbolType};
use crate::parser::directive::Directive;
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
    code_section: Vec<u8>,
    symbols: SymbolTable,
    current_section: Option<AssemblerSection>,
    next_alignment: Option<usize>,
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
                    self.handle_directive_first_pass(directive, &mut offset)?;
                }
            }
        }

        Ok(())
    }

    /// Handles any directives encountered in the first pass
    fn handle_directive_first_pass(
        &mut self,
        directive: &DirectiveInstruction,
        offset: &mut u32,
    ) -> Result<(), AssemblerError> {
        // no operands, so treat as section
        if directive.operands.is_empty() {
            self.current_section = Some(AssemblerSection::from(directive.directive));
            return Ok(());
        }

        // directive with label, so first check we're in a section
        if self.current_section.is_none() {
            return Err(AssemblerError::NoSegmentDeclarationFound);
        }

        match directive.directive {
            Directive::Align => {
                // if alignment, set the next alignment value to first argument
                if let Some(&Operand::Value(value)) = directive.operands.first() {
                    self.next_alignment = Some(value as usize);
                }
            }
            Directive::Ascii
            | Directive::Asciiz
            | Directive::Byte
            | Directive::Half
            | Directive::Word
            | Directive::Space => {
                // add label if it exists
                if let Some(label) = &directive.label {
                    // then add the symbol, returning error if it already exists
                    if !self
                        .symbols
                        .add_symbol(label, Symbol::new(*offset, SymbolType::Label))
                    {
                        return Err(AssemblerError::SymbolAlreadyDeclared);
                    }
                }
            }
            _ => {}
        }

        // skip align directive since works different
        if directive.directive != Directive::Align {
            // finally move offset by size of directive
            *offset += directive.size(self.next_alignment.take()) as u32;
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
                                    let offset = symbol.offset as u16 + PIE_HEADER_LENGTH as u16;

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
                    self.handle_directive_second_pass(directive)?;
                }
            }
        }
        Ok(())
    }

    fn handle_directive_second_pass(
        &mut self,
        directive: &DirectiveInstruction,
    ) -> Result<(), AssemblerError> {
        // no operands, so treat as section
        if directive.operands.is_empty() {
            self.current_section = Some(AssemblerSection::from(directive.directive));

            return Ok(());
        }

        match directive.directive {
            Directive::Align => {
                if let Some(&Operand::Value(value)) = directive.operands.first() {
                    self.next_alignment = Some(value as usize);
                }
            }
            Directive::Ascii
            | Directive::Asciiz
            | Directive::Byte
            | Directive::Half
            | Directive::Word
            | Directive::Space => {
                let bytes = directive.aligned_bytes(self.next_alignment.take());

                match (&self.current_section, bytes) {
                    (Some(AssemblerSection::Data), Some(bytes)) => {
                        self.data_section.extend_from_slice(&bytes)
                    }
                    (Some(AssemblerSection::Code), Some(bytes)) => {
                        self.data_section.extend_from_slice(&bytes)
                    }
                    _ => return Err(AssemblerError::NoSegmentDeclarationFound),
                }
            }

            _ => {}
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
                                    hello: .ascii 'Hell'
                                    world: .asciiz 'world!'
                                .code
                                    inc $5
                                    loop:
                                    inc $5
                                    djmp @loop"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 12, 0, 0, 0, 76, 0, 0, 0, 12, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [72, 101, 108, 108, 119, 111, 114, 108, 100, 33, 0, 0];
        let expected_code = [19, 5, 0, 0, 19, 5, 0, 0, 21, 0, 80, 0];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .chain(expected_code.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }

    #[test]
    fn test_alignment() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    .align 8
                                    a: .asciiz 'a'
                                    .align 2
                                    b: .ascii 'a'
                                    c: .ascii 'ab'
                                .code"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 14, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [97, 0, 0, 0, 0, 0, 0, 0, 97, 0, 97, 98, 0, 0];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }

    #[test]
    fn test_byte() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    a: .byte 1, 2, 3, 4, 5
                                    .align 2
                                    b: .byte 1
                                .code"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 10, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [1, 2, 3, 4, 5, 0, 0, 0, 1, 0];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }

    #[test]
    fn test_half() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    a: .half 100, 200, 300
                                    .align 2
                                    b: .half 256
                                .code"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 10, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [0, 100, 0, 200, 1, 44, 0, 0, 1, 0];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }

    #[test]
    fn test_word() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    a: .word -2147483648, 2147483647
                                    .align 8
                                    b: .word 2147483647
                                .code"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 16, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [
            128, 0, 0, 0, 127, 255, 255, 255, 127, 255, 255, 255, 0, 0, 0, 0,
        ];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }

    #[test]
    fn test_space() {
        let mut asm = Assembler::default();
        let program = r#".data
                                    .align 1
                                    a: .byte 1
                                    .space 6
                                    .align 1
                                    b: .byte 1
                                .code"#;
        let expected_header = [
            69, 80, 73, 69, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 8, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected_data = [1, 0, 0, 0, 0, 0, 0, 1];

        let expected: Vec<u8> = expected_header
            .into_iter()
            .chain(expected_data.into_iter())
            .collect();

        let program = asm.assemble(program).unwrap();
        assert_eq!(program, expected);
    }
}
