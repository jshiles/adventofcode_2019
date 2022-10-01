pub struct Instruction {
    pub opcode: i32,
    pub parameters: Vec<i32>,
}

pub trait Size {
    fn size(&self) -> usize;
}

impl Size for Instruction {
    fn size(&self) -> usize {
        1 + self.parameters.len()
    }
}

fn initialize_instruction(memory: &Vec<i32>, addr: usize) -> Instruction {
    if memory[addr] == 99 {
        return Instruction {
            opcode: memory[addr],
            parameters: vec![],
        }
    } else if memory[addr] == 1 || memory[addr] == 2 {
        return Instruction {
            opcode: memory[addr],
            parameters: vec![
                memory[memory[addr+1] as usize], // first number
                memory[memory[addr+2] as usize], // second number
                memory[addr+3], // target address
            ],
        }
    } else if memory[addr] == 3 || memory[addr] == 4 {
        return Instruction {
            opcode: memory[addr],
            parameters: vec![
                memory[memory[addr+1] as usize],
            ],
        }
    }
    Instruction { opcode: memory[addr], parameters: vec![] } // fallback
}


pub fn run_intcode(mut memory: Vec<i32>, start_addr: usize, input: i32, output: i32, mode: i32) -> Vec<i32> {

    let instr: Instruction = initialize_instruction(&memory, start_addr);
    if instr.opcode == 99 {
        return memory
    } else {
        match instr.opcode {
            1 => memory[instr.parameters[2] as usize] = instr.parameters[0] + instr.parameters[1],
            2 => memory[instr.parameters[2] as usize] = instr.parameters[0] * instr.parameters[1],
            3 => (), // TODO: implement me
            4 => (), // TODO: implement me
            _other => println!("Error! Invalid Op Code! {:?}", 
                memory[start_addr]),  
        };
        run_intcode(memory, start_addr+instr.size(), input, output, mode)
    }
}

#[macro_export]
macro_rules! run_intcode {
    ($memory: expr) => {
        run_intcode($memory, 0, 0, 0, 0)
    };
    ($memory: expr, $addr: expr, $input: expr, $output: expr) => {
        run_intcode($memory, $addr, $input, $output, 0)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02 () {
        let test_initcode_before: Vec<i32> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let test_initcode_after = run_intcode!(test_initcode_before, 0, 0, 0);
        assert_eq!(test_initcode_after[0], 3500);

        let test_initcode_before_2: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        let test_initcode_after_2 = run_intcode!(test_initcode_before_2);
        assert_eq!(test_initcode_after_2[0], 30);
    }

    #[test]
    fn test_day_05 () {
        assert_eq!(true, true);
    }
}