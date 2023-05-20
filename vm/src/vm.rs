use crate::instruction::Instruction;
use shared::Opcode;
use shared::PIE_HEADER_PREFIX;

/// Main virtual machine
#[derive(Default)]
pub struct VM {
    /// CPU Registers
    pub registers: [i32; 32],
    /// Program counter - current byte being executed
    pc: usize,
    /// Program to be executed
    pub program: Vec<u8>,
    /// Start of bytecode section
    code_section_start: usize,
    /// Remainder from previous instruction
    remainder: u32,
    /// Equality from last comparison instruction
    pub equality_flag: bool,
}

impl VM {
    fn verify_header(&self) -> bool {
        self.program[0..4] == PIE_HEADER_PREFIX
    }

    /// Runs VM until completion
    pub fn run(&mut self) {
        // test header and then skip to code section
        if !self.verify_header() {}
        self.code_section_start =
            u32::from_be_bytes(self.program[16..20].try_into().unwrap()) as usize;

        self.pc = self.code_section_start;

        while self.execute_instruction() {}
    }

    /// Runs the VM, executing a single instruction
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Executes a single instruction, returning a bool indicating if another instruction can be ran
    /// afterwards
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        // read 4 bytes and advance PC
        let mut instruction =
            if let Some(inst) = Instruction::from(&self.program[self.pc..self.pc + 4]) {
                inst
            } else {
                return false;
            };
        self.pc += 4;

        match instruction.opcode {
            Opcode::HLT => {
                println!("Halting!");
                return false;
            }
            Opcode::LDBI => {
                let register = instruction.next_u8() as usize;
                let value = instruction.next_u16() as u8 as i32;

                self.registers[register] = value;
            }
            Opcode::LDBD => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_u16() as usize;

                self.registers[register] = self.program[address] as i32;
            }
            Opcode::LDBR => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_register(&self.registers) as usize;

                self.registers[register] = self.program[address] as i32;
            }
            Opcode::LDHI => {
                let register = instruction.next_u8() as usize;
                let value = instruction.next_u16() as i32;

                self.registers[register] = value;
            }
            Opcode::LDHD => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_u16() as usize;

                let bytes = [self.program[address], self.program[address + 1]];

                self.registers[register] = i16::from_be_bytes(bytes) as i32;
            }
            Opcode::LDHR => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_register(&self.registers) as usize;

                let bytes = [self.program[address], self.program[address + 1]];

                self.registers[register] = i16::from_be_bytes(bytes) as i32;
            }
            Opcode::LDWD => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_u16() as usize;

                let bytes = [
                    self.program[address],
                    self.program[address + 1],
                    self.program[address + 2],
                    self.program[address + 3],
                ];

                self.registers[register] = i32::from_be_bytes(bytes);
            }
            Opcode::LDWR => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_register(&self.registers) as usize;

                let bytes = [
                    self.program[address],
                    self.program[address + 1],
                    self.program[address + 2],
                    self.program[address + 3],
                ];

                self.registers[register] = i32::from_be_bytes(bytes);
            }
            Opcode::STRBI => {
                let register = instruction.next_register(&self.registers) as u8;
                let address = instruction.next_u16() as usize;

                self.program[address] = register;
            }
            Opcode::STRBR => {
                let register = instruction.next_register(&self.registers) as u8;
                let address = instruction.next_register(&self.registers) as usize;

                self.program[address] = register;
            }
            Opcode::STRHI => {
                let register = instruction.next_register(&self.registers) as u16;
                let mut address = instruction.next_u16() as usize;

                for byte in register.to_be_bytes() {
                    self.program[address] = byte;
                    address += 1;
                }
            }
            Opcode::STRHR => {
                let register = instruction.next_register(&self.registers) as u16;
                let mut address = instruction.next_register(&self.registers) as usize;

                for byte in register.to_be_bytes() {
                    self.program[address] = byte;
                    address += 1;
                }
            }
            Opcode::STRWI => {
                let register = instruction.next_register(&self.registers) as u32;
                let mut address = instruction.next_u16() as usize;

                for byte in register.to_be_bytes() {
                    self.program[address] = byte;
                    address += 1;
                }
            }
            Opcode::STRWR => {
                let register = instruction.next_register(&self.registers) as u32;
                let mut address = instruction.next_register(&self.registers) as usize;

                for byte in register.to_be_bytes() {
                    self.program[address] = byte;
                    address += 1;
                }
            }
            Opcode::MOV => {
                let register_a = instruction.next_u8() as usize;
                let register_b = instruction.next_register(&self.registers);

                self.registers[register_a] = register_b;
            }
            Opcode::ADDR => {
                let register_a = instruction.next_u8() as usize;
                let register_b = instruction.next_register(&self.registers);
                let register_c = instruction.next_register(&self.registers);

                self.registers[register_a] = register_b + register_c;
            }
            Opcode::ADDI => {
                let register_a = instruction.next_u8() as usize;
                let value = instruction.next_u16() as i32;

                self.registers[register_a] += value;
            }
            Opcode::SUBR => {
                let register_a = instruction.next_u8() as usize;
                let register_b = instruction.next_register(&self.registers);
                let register_c = instruction.next_register(&self.registers);

                self.registers[register_a] = register_b - register_c;
            }
            Opcode::SUBI => {
                let register_a = instruction.next_u8() as usize;
                let value = instruction.next_u16() as i32;

                self.registers[register_a] -= value;
            }
            Opcode::MULR => {
                let register_a = instruction.next_u8() as usize;
                let register_b = instruction.next_register(&self.registers);
                let register_c = instruction.next_register(&self.registers);

                self.registers[register_a] = register_b * register_c;
            }
            Opcode::MULI => {
                let register_a = instruction.next_u8() as usize;
                let value = instruction.next_u16() as i32;

                self.registers[register_a] *= value;
            }
            Opcode::DIVR => {
                let register_a = instruction.next_u8() as usize;
                let register_b = instruction.next_register(&self.registers);
                let register_c = instruction.next_register(&self.registers);

                let (value, remainder) = (register_b / register_c, register_b % register_c);

                self.registers[register_a] = value;
                self.remainder = remainder as u32;
            }
            Opcode::DIVI => {
                let register_addr = instruction.next_u8() as usize;
                let register_value = self.registers[register_addr];
                let value = instruction.next_u16() as i32;

                let (value, remainder) = (register_value / value, register_value % value);

                self.registers[register_addr] = value;
                self.remainder = remainder as u32;
            }
            Opcode::EQI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register == value as i32;
            }
            Opcode::EQR => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a == register_b;
            }
            Opcode::NEQI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register != value as i32;
            }
            Opcode::NEQR => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a != register_b;
            }
            Opcode::GTI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register > value as i32;
            }
            Opcode::GTR => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a > register_b;
            }
            Opcode::GTEI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register >= value as i32;
            }
            Opcode::GTER => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a >= register_b;
            }
            Opcode::LTI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register < value as i32;
            }
            Opcode::LTR => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a < register_b;
            }
            Opcode::LTEI => {
                let register = instruction.next_register(&self.registers);
                let value = instruction.next_u16();

                self.equality_flag = register <= value as i32;
            }
            Opcode::LTER => {
                let register_a = instruction.next_register(&self.registers);
                let register_b = instruction.next_register(&self.registers);

                self.equality_flag = register_a <= register_b;
            }
            Opcode::JMPI => {
                self.pc = instruction.next_u16() as usize;
            }
            Opcode::JMPD => {
                let address = instruction.next_u16() as usize;
                let bytes = [
                    self.program[address],
                    self.program[address + 1],
                    self.program[address + 2],
                    self.program[address + 3],
                ];

                self.pc = u32::from_be_bytes(bytes) as usize;
            }
            Opcode::JMPR => {
                self.pc = instruction.next_register(&self.registers) as usize;
            }
            Opcode::JMPEI => {
                if self.equality_flag {
                    self.pc = instruction.next_u16() as usize;
                }
            }
            Opcode::JMPED => {
                if self.equality_flag {
                    let address = instruction.next_u16() as usize;
                    let bytes = [
                        self.program[address],
                        self.program[address + 1],
                        self.program[address + 2],
                        self.program[address + 3],
                    ];

                    self.pc = u32::from_be_bytes(bytes) as usize;
                }
            }
            Opcode::JMPER => {
                if self.equality_flag {
                    self.pc = instruction.next_register(&self.registers) as usize;
                }
            }
            Opcode::JMPNEI => {
                if !self.equality_flag {
                    self.pc = instruction.next_u16() as usize;
                }
            }
            Opcode::JMPNED => {
                if !self.equality_flag {
                    let address = instruction.next_u16() as usize;
                    let bytes = [
                        self.program[address],
                        self.program[address + 1],
                        self.program[address + 2],
                        self.program[address + 3],
                    ];

                    self.pc = u32::from_be_bytes(bytes) as usize;
                }
            }
            Opcode::JMPNER => {
                if !self.equality_flag {
                    self.pc = instruction.next_register(&self.registers) as usize;
                }
            }
            Opcode::PRTSD => {
                let start = instruction.next_u16() as usize;
                let mut end = start;

                while self.program[end] != 0 {
                    end += 1;
                }

                let string = std::str::from_utf8(&self.program[start..end]);
                if let Ok(string) = string {
                    println!("{string}");
                } else {
                    println!("Invalid string!");
                }
            }
            Opcode::PRTSR => {
                let start = instruction.next_register(&self.registers) as usize;
                let mut end = start;

                while self.program[end] != 0 {
                    end += 1;
                }

                let string = std::str::from_utf8(&self.program[start..end]);
                if let Ok(string) = string {
                    println!("{string}");
                } else {
                    println!("Invalid string!");
                }
            }
            _ => {
                println!("Unrecognized opcode encountered");
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::PIE_HEADER_LENGTH;

    fn get_test_vm(program: Vec<u8>) -> VM {
        let mut registers = [0; 32];
        registers[0] = 5;
        registers[1] = 10;

        VM {
            registers,
            program,
            ..Default::default()
        }
    }

    fn prepend_header(vm: &mut VM) {
        let mut out = Vec::with_capacity(PIE_HEADER_LENGTH);

        out.extend_from_slice(&PIE_HEADER_PREFIX);
        out.extend_from_slice(&[
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            PIE_HEADER_LENGTH as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            PIE_HEADER_LENGTH as u8,
            0,
            0,
            0,
            0,
        ]);
        if out.len() < PIE_HEADER_LENGTH {
            out.resize(PIE_HEADER_LENGTH, 0);
        }

        out.extend_from_slice(&vm.program);
        vm.program = out;
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::default();

        assert_eq!(test_vm.registers, [0; 32]);
        assert_eq!(test_vm.pc, 0);
        assert_eq!(test_vm.program, vec![]);
    }

    macro_rules! opcode_test {
        (
            $name:ident; $vm_name:ident;
            [$( $program:expr ),*],
            $($test_left:expr => $test_right:expr),*
            $(; $($prep_left:expr => $prep_right:expr),*)?
        ) => {
            #[test]
            fn $name() {
                let mut $vm_name = get_test_vm(vec![$($program),*]);
                prepend_header(&mut $vm_name);
                $($(
                    $prep_left = $prep_right;
                )?);*

                $vm_name.run();

                $(
                    assert_eq!($test_left, $test_right)
                );*
            }
        };
    }

    // misc instructions
    opcode_test!(test_opcode_hlt; vm; [0, 0, 0, 0, 1, 0, 0, 0], vm.pc => 68);
    opcode_test!(test_opcode_igl; vm; [0x3F, 0, 0, 0, 0, 0, 0, 0], vm.pc => 68);

    // load instructions
    opcode_test!(test_opcode_ldbi; vm; [4, 0, 255, 255], vm.registers[0] => 0xFF);
    opcode_test!(test_opcode_ldbd; vm; [5, 0, 0, 0], vm.registers[0] => 0x45);
    opcode_test!(test_opcode_ldbr; vm; [6, 0, 0, 0], vm.registers[0] => 0xAB; vm.program[5] => 0xAB);
    opcode_test!(test_opcode_ldhi; vm; [8, 0, 255, 255], vm.registers[0] => 0xFFFF);
    opcode_test!(test_opcode_ldhd; vm; [9, 0, 0, 0], vm.registers[0] => 0x4550);
    opcode_test!(test_opcode_ldhr; vm; [10, 0, 0, 0], vm.registers[0] => -21555; vm.program[5] => 0xAB, vm.program[6] => 0xCD);
    opcode_test!(test_opcode_ldwd; vm; [13, 0, 0, 0], vm.registers[0] => 0x45504945);
    opcode_test!(test_opcode_ldwr; vm; [14, 0, 0, 0], vm.registers[0] => 0x40ABCDEF; vm.program[5] => 0x40, vm.program[6] => 0xAB, vm.program[7] => 0xCD, vm.program[8] => 0xEF);

    // store/move instructions
    opcode_test!(test_opcode_strbi; vm; [16, 1, 0, 0], &vm.program[0..4] => [10, 0x50, 0x49, 0x45]);
    opcode_test!(test_opcode_strbr; vm; [18, 1, 0, 0], &vm.program[5..9] => [10, 0, 0, 0]);
    opcode_test!(test_opcode_strhi; vm; [20, 1, 0, 0], &vm.program[0..4] => [0, 10, 0x49, 0x45]);
    opcode_test!(test_opcode_strhr; vm; [22, 1, 0, 0], &vm.program[5..9] => [0, 10, 0, 0]);
    opcode_test!(test_opcode_strwi; vm; [24, 1, 0, 0], &vm.program[0..4] => [0, 0, 0, 10]);
    opcode_test!(test_opcode_strwr; vm; [26, 1, 0, 0], &vm.program[5..9] => [0, 0, 0, 10]);
    opcode_test!(test_opcode_mov; vm; [30, 0, 1, 0], vm.registers[0] => 10);

    // arithmetic instructions
    opcode_test!(test_opcode_adr; vm; [66, 2, 0, 1], vm.registers[2] => 15);
    opcode_test!(test_opcode_adi; vm; [64, 0, 1, 2], vm.registers[0] => 263);
    opcode_test!(test_opcode_sur; vm; [70, 2, 1, 0], vm.registers[2] => 5);
    opcode_test!(test_opcode_sui; vm; [68, 0, 0, 4], vm.registers[0] => 1);
    opcode_test!(test_opcode_mlr; vm; [74, 2, 1, 0], vm.registers[2] => 50);
    opcode_test!(test_opcode_mli; vm; [72, 0, 0, 4], vm.registers[0] => 20);
    opcode_test!(test_opcode_dvr; vm; [78, 2, 1, 0], vm.registers[2] => 2, vm.remainder => 0);
    opcode_test!(test_opcode_dvi; vm; [76, 0, 0, 4], vm.registers[0] => 1, vm.remainder => 1);

    // comparison instructions
    opcode_test!(test_opcode_eqi; vm; [128, 0, 0, 5], vm.equality_flag => true);
    opcode_test!(test_opcode_eqr; vm; [130, 0, 1, 0], vm.equality_flag => false);
    opcode_test!(test_opcode_neqi; vm; [132, 0, 0, 5], vm.equality_flag => false);
    opcode_test!(test_opcode_neqr; vm; [134, 0, 1, 0], vm.equality_flag => true);

    opcode_test!(test_opcode_gti; vm; [136, 0, 0, 1], vm.equality_flag => true);
    opcode_test!(test_opcode_gtr; vm; [138, 1, 0, 0], vm.equality_flag => true);
    opcode_test!(test_opcode_gtei; vm; [140, 0, 0, 5], vm.equality_flag => true);
    opcode_test!(test_opcode_gter; vm; [142, 1, 0, 0], vm.equality_flag => true);

    opcode_test!(test_opcode_lti; vm; [144, 0, 0, 1], vm.equality_flag => false);
    opcode_test!(test_opcode_ltr; vm; [146, 0, 1, 0], vm.equality_flag => true);
    opcode_test!(test_opcode_ltei; vm; [148, 0, 0, 5], vm.equality_flag => true);
    opcode_test!(test_opcode_lter; vm; [150, 1, 0, 0], vm.equality_flag => false);

    // jump instructions
    opcode_test!(test_opcode_jmpi; vm; [160, 1, 0, 0], vm.pc => 256);
    opcode_test!(test_opcode_jmpd; vm; [161, 0, 0, 0], vm.pc => u32::from_be_bytes(PIE_HEADER_PREFIX) as usize);
    opcode_test!(test_opcode_jmpr; vm; [8, 1, 1, 0, 162, 1, 0, 0], vm.pc => 256);

    opcode_test!(test_opcode_jmpei_a; vm; [164, 1, 0, 0], vm.pc => 68; vm.equality_flag => false);
    opcode_test!(test_opcode_jmpei_b; vm; [164, 1, 0, 0], vm.pc => 256; vm.equality_flag => true);
    opcode_test!(test_opcode_jmped_a; vm; [165, 0, 0, 0], vm.pc => 68; vm.equality_flag => false);
    opcode_test!(test_opcode_jmped_b; vm; [165, 0, 0, 0], vm.pc => u32::from_be_bytes(PIE_HEADER_PREFIX) as usize; vm.equality_flag => true);
    opcode_test!(test_opcode_jmper_a; vm; [8, 1, 1, 0, 166, 1, 0, 0], vm.pc => 72; vm.equality_flag => false);
    opcode_test!(test_opcode_jmper_b; vm; [8, 1, 1, 0, 166, 1, 0, 0], vm.pc => 256; vm.equality_flag => true);

    opcode_test!(test_opcode_jmpnei_a; vm; [168, 1, 0, 0], vm.pc => 68; vm.equality_flag => true);
    opcode_test!(test_opcode_jmpnei_b; vm; [168, 1, 0, 0], vm.pc => 256; vm.equality_flag => false);
    opcode_test!(test_opcode_jmpned_a; vm; [169, 0, 0, 0], vm.pc => 68; vm.equality_flag => true);
    opcode_test!(test_opcode_jmpned_b; vm; [169, 0, 0, 0], vm.pc => u32::from_be_bytes(PIE_HEADER_PREFIX) as usize; vm.equality_flag => false);
    opcode_test!(test_opcode_jmpner_a; vm; [8, 1, 1, 0, 170, 1, 0, 0], vm.pc => 72; vm.equality_flag => true);
    opcode_test!(test_opcode_jmpner_b; vm; [8, 1, 1, 0, 170, 1, 0, 0], vm.pc => 256; vm.equality_flag => false);
}
