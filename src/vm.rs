use crate::opcode::Opcode;

/// Main virtual machine struct
pub struct VM {
    /// CPU Registers
    registers: [i32; 32],
    /// Program counter - current byte being executed
    pc: usize,
    /// Program to be executed
    program: Vec<u8>,
    /// Remainder from previous instruction
    remainder: u32,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }
}

impl VM {
    /// Decodes the opcode at the current PC, and increments PC by one
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    /// Returns the next 8 bits in program, and increments PC by one
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    /// Returns the next 16 bits in program, and increments PC by two
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;

        result
    }

    /// Runs VM until completion
    pub fn run(&mut self) {
        while !self.execute_instruction() {}
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

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_1 + register_2;
            }
            Opcode::SUB => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_1 - register_2;
            }
            Opcode::MUL => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register_1 * register_2;
            }
            Opcode::DIV => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                let (div, rem) = (register_1 / register_2, register_1 % register_2);
                self.registers[self.next_8_bits() as usize] = div;
                self.remainder = rem as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];

                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let offset = self.registers[self.next_8_bits() as usize];

                self.pc += offset as usize;
            }
            Opcode::JMPB => {
                let offset = self.registers[self.next_8_bits() as usize];

                self.pc -= offset as usize;
            }
            _ => {
                println!("Unrecognized opcode encountered");
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_create_vm() {
        let test_vm = VM::default();

        assert_eq!(test_vm.registers, [0; 32]);
        assert_eq!(test_vm.pc, 0);
        assert_eq!(test_vm.program, vec![]);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM {
            program: vec![6, 0, 0, 0],
            ..Default::default()
        };

        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM {
            program: vec![200, 0, 0, 0],
            ..Default::default()
        };

        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM {
            program: vec![0, 0, 1, 244],
            ..Default::default()
        };

        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm(vec![1, 0, 1, 2]);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm(vec![2, 1, 0, 2]);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm(vec![3, 0, 1, 2]);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm(vec![4, 1, 0, 2]);
        test_vm.registers[1] = 11;
        test_vm.run();

        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm(vec![7, 0, 0, 0]);
        test_vm.registers[0] = 0;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm(vec![8, 0, 0, 0, 6, 0, 0, 0]);
        test_vm.registers[0] = 2;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm(vec![0, 0, 0, 10, 9, 1, 0, 0]);
        test_vm.registers[1] = 6;
        test_vm.run_once();
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }
}
