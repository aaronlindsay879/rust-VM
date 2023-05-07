use crate::opcode::Opcode;

/// Main virtual machine struct
pub struct VM {
    /// CPU Registers
    pub(crate) registers: [i32; 32],
    /// Program counter - current byte being executed
    pc: usize,
    /// Program to be executed
    pub(crate) program: Vec<u8>,
    /// Remainder from previous instruction
    remainder: u32,
    /// Equality from last comparison instruction
    pub(crate) equality_flag: bool,
    /// Heap memory
    heap: Vec<u8>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equality_flag: false,
            heap: vec![],
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

    /// Returns the value of the register specified in the next 8 bits, and increments PC by one
    fn next_register(&mut self) -> i32 {
        self.registers[self.next_8_bits() as usize]
    }

    /// Returns a mutable reference to the register specified in the next 8 bits,
    /// and increments PC by one
    fn next_register_mut(&mut self) -> &mut i32 {
        &mut self.registers[self.next_8_bits() as usize]
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
                let number = self.next_16_bits();

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                *self.next_register_mut() = register_1 + register_2;
            }
            Opcode::SUB => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                *self.next_register_mut() = register_1 - register_2;
            }
            Opcode::MUL => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                *self.next_register_mut() = register_1 * register_2;
            }
            Opcode::DIV => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                let (div, rem) = (register_1 / register_2, register_1 % register_2);
                *self.next_register_mut() = div;
                self.remainder = rem as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
            }
            Opcode::JMP => {
                let target = self.next_register();

                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let offset = self.next_register();

                self.pc += offset as usize;
            }
            Opcode::JMPB => {
                let offset = self.next_register();

                self.pc -= offset as usize;
            }
            Opcode::EQ => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 == register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::NEQ => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 != register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::GTE => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 >= register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::GT => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 > register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::LTE => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 <= register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::LT => {
                let register_1 = self.next_register();
                let register_2 = self.next_register();

                self.equality_flag = register_1 < register_2;
                self.next_8_bits(); // eat next byte to pad instruction size
            }
            Opcode::JMPE => {
                let target = self.next_register();

                if self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JMPNE => {
                let target = self.next_register();

                if !self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::NOP => {
                // eat next 3 bytes
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                let bytes = self.next_register();
                self.heap.resize(self.heap.len() + bytes as usize, 0);
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
            program: vec![5, 0, 0, 0],
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
        let mut test_vm = get_test_vm(vec![6, 0, 0, 0]);
        test_vm.registers[0] = 0;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm(vec![7, 0, 0, 0, 6, 0, 0, 0]);
        test_vm.registers[0] = 2;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm(vec![0, 0, 0, 10, 8, 1, 0, 0]);
        test_vm.registers[1] = 6;
        test_vm.run_once();
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm(vec![9, 0, 1, 0, 9, 0, 1, 0]);
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm(vec![10, 0, 1, 0, 10, 0, 1, 0]);
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);
    }

    #[test]
    fn test_gte_opcode() {
        let mut test_vm = get_test_vm(vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0]);
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = get_test_vm(vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0]);
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);

        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = get_test_vm(vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0]);
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);

        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = get_test_vm(vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0]);
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);

        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, false);

        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equality_flag, true);
    }

    #[test]
    fn test_jmpe_opcode() {
        let mut test_vm = get_test_vm(vec![15, 0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0]);
        test_vm.registers[0] = 7;
        test_vm.equality_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_jmpne_opcode() {
        let mut test_vm = get_test_vm(vec![16, 0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0]);
        test_vm.registers[0] = 7;
        test_vm.equality_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = get_test_vm(vec![18, 0, 0, 0]);
        test_vm.registers[0] = 1024;
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }
}
