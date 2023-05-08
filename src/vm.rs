use crate::instruction::Instruction;
use crate::opcode::Opcode;
use crate::PIE_HEADER_PREFIX;

/// Main virtual machine
#[derive(Default)]
pub struct VM {
    /// CPU Registers
    pub(crate) registers: [i32; 32],
    /// Program counter - current byte being executed
    pc: usize,
    /// Program to be executed
    pub(crate) program: Vec<u8>,
    /// Start of bytecode section
    code_section_start: usize,
    /// Remainder from previous instruction
    remainder: u32,
    /// Equality from last comparison instruction
    pub(crate) equality_flag: bool,
    /// Heap memory
    heap: Vec<u8>,
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
            Opcode::LBI => {
                let register = instruction.next_u8() as usize;
                let value = instruction.next_u8() as i32;

                self.registers[register] = value;
            }
            Opcode::LBD => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_u16() as usize;

                self.registers[register] = self.program[address] as i32;
            }
            Opcode::LHI => {
                let register = instruction.next_u8() as usize;
                let value = instruction.next_u16() as i32;

                self.registers[register] = value;
            }
            Opcode::LHD => {
                let register = instruction.next_u8() as usize;
                let address = instruction.next_u16() as usize;

                let bytes = [self.program[address], self.program[address + 1]];

                self.registers[register] = i16::from_be_bytes(bytes) as i32;
            }
            Opcode::LWD => {
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
    use crate::PIE_HEADER_LENGTH;

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
        ($name:ident; $vm_name:ident; [$( $program:expr ),*], $($test_left:expr => $test_right:expr),*) => {
            #[test]
            fn $name() {
                let mut $vm_name = get_test_vm(vec![$($program),*]);
                prepend_header(&mut $vm_name);
                $vm_name.run();

                $(
                    assert_eq!($test_left, $test_right);
                );*
            }
        };
    }

    opcode_test!(test_opcode_hlt; vm; [0, 0, 0, 0, 1, 0, 0, 0], vm.pc => 68);
    opcode_test!(test_opcode_igl; vm; [0x3F, 0, 0, 0, 0, 0, 0, 0], vm.pc => 68);

    opcode_test!(test_opcode_lbi; vm; [4, 0, 255, 255], vm.registers[0] => 0xFF);
    opcode_test!(test_opcode_lbd; vm; [5, 0, 0, 0], vm.registers[0] => 0x45);
    opcode_test!(test_opcode_lhi; vm; [8, 0, 255, 255], vm.registers[0] => 0xFFFF);
    opcode_test!(test_opcode_lhd; vm; [9, 0, 0, 0], vm.registers[0] => 0x4550);
    opcode_test!(test_opcode_lwd; vm; [13, 0, 0, 0], vm.registers[0] => 0x45504945);
}
