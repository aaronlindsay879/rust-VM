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
            Opcode::LOAD => {
                let register = instruction.next_u8() as usize;
                let number = instruction.next_u16();

                self.registers[register] = number as i32;
            }
            Opcode::STORE => {
                let register = instruction.next_register(&self.registers);
                let location = instruction.next_u16() as usize;

                self.program[location] = register as u8;
            }
            Opcode::ADD => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                *instruction.next_register_mut(&mut self.registers) = register_1 + register_2;
            }
            Opcode::SUB => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                *instruction.next_register_mut(&mut self.registers) = register_1 - register_2;
            }
            Opcode::MUL => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                *instruction.next_register_mut(&mut self.registers) = register_1 * register_2;
            }
            Opcode::DIV => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                let (div, rem) = (register_1 / register_2, register_1 % register_2);
                *instruction.next_register_mut(&mut self.registers) = div;
                self.remainder = rem as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::JMP => {
                let target = instruction.next_register(&self.registers);

                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let offset = instruction.next_register(&self.registers);

                self.pc += offset as usize;
            }
            Opcode::JMPB => {
                let offset = instruction.next_register(&self.registers);

                self.pc -= offset as usize;
            }
            Opcode::EQ => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 == register_2;
            }
            Opcode::NEQ => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 != register_2;
            }
            Opcode::GTE => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 >= register_2;
            }
            Opcode::GT => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 > register_2;
            }
            Opcode::LTE => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 <= register_2;
            }
            Opcode::LT => {
                let register_1 = instruction.next_register(&self.registers);
                let register_2 = instruction.next_register(&self.registers);

                self.equality_flag = register_1 < register_2;
            }
            Opcode::JMPE => {
                let target = instruction.next_register(&self.registers);

                if self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JMPNE => {
                let target = instruction.next_register(&self.registers);

                if !self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::NOP => {}
            Opcode::ALOC => {
                let bytes = instruction.next_register(&self.registers);
                self.heap.resize(self.heap.len() + bytes as usize, 0);
            }
            Opcode::INC => {
                *instruction.next_register_mut(&mut self.registers) += 1;
            }
            Opcode::DEC => {
                *instruction.next_register_mut(&mut self.registers) -= 1;
            }
            Opcode::DJMP => {
                let target = instruction.next_u16();

                self.pc = target as usize;
            }
            Opcode::DJMPE => {
                let target = instruction.next_u16();

                if self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::DJMPNE => {
                let target = instruction.next_u16();

                if !self.equality_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::PRTS => {
                let offset = instruction.next_u16() as usize;
                let slice = self
                    .program
                    .iter()
                    .skip(offset)
                    .take_while(|&&byte| byte != 0)
                    .copied()
                    .collect::<Vec<_>>();

                match std::str::from_utf8(&slice) {
                    Ok(s) => println!("{s}"),
                    Err(e) => println!("Error decoding string: {e:?}"),
                };
            }
            Opcode::LOADM => {
                let location = instruction.next_register(&self.registers) as usize;
                let data = {
                    let slice = &self.heap[location..location + 4];
                    i32::from_be_bytes(slice.try_into().unwrap())
                };

                *instruction.next_register_mut(&mut self.registers) = data;
            }
            Opcode::SETM => {
                let location = instruction.next_register(&self.registers) as usize;
                let data = instruction.next_register(&self.registers);

                for (mem, byte) in self.heap[location..location + 4]
                    .iter_mut()
                    .zip(data.to_be_bytes())
                {
                    *mem = byte;
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

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM {
            program: vec![5, 0, 0, 0],
            ..Default::default()
        };
        prepend_header(&mut test_vm);

        test_vm.run();

        assert_eq!(test_vm.pc, 68);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM {
            program: vec![200, 0, 0, 0],
            ..Default::default()
        };
        prepend_header(&mut test_vm);

        test_vm.run();

        assert_eq!(test_vm.pc, 68);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM {
            program: vec![0, 0, 1, 244],
            ..Default::default()
        };
        prepend_header(&mut test_vm);

        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm(vec![1, 0, 1, 2]);
        prepend_header(&mut test_vm);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm(vec![2, 1, 0, 2]);
        prepend_header(&mut test_vm);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm(vec![3, 0, 1, 2]);
        prepend_header(&mut test_vm);
        test_vm.run();

        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm(vec![4, 1, 0, 2]);
        test_vm.registers[1] = 11;
        prepend_header(&mut test_vm);
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
        test_vm.registers[0] = 4;
        test_vm.run_once();

        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm(vec![0, 0, 0, 10, 8, 1, 0, 0]);
        test_vm.registers[1] = 8;
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
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = get_test_vm(vec![18, 0, 0, 0]);
        test_vm.registers[0] = 1024;
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_inc_opcode() {
        let mut test_vm = get_test_vm(vec![19, 0, 0, 0]);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 6);
    }

    #[test]
    fn test_dec_opcode() {
        let mut test_vm = get_test_vm(vec![20, 0, 0, 0]);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 4);
    }
}
