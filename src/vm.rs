use crate::opcode::Opcode;

/// Main virtual machine struct
pub struct VM {
    /// CPU Registers
    registers: [i32; 32],
    /// Program counter - current byte being executed
    pc: usize,
    /// Program to be executed
    program: Vec<u8>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: vec![],
        }
    }
}

impl VM {
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    /// Main VM loop
    pub fn run(&mut self) {
        loop {
            // make sure PC still in program bounds
            if self.pc >= self.program.len() {
                break;
            }

            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered");
                    break;
                }
                _ => {
                    println!("Unrecognized opcode encountered");
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            program: vec![0, 0, 0, 0],
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
}
