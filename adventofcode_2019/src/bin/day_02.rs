/* 
https://adventofcode.com/2019/day/2
john shiles 
*/

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use itertools::Itertools;

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

fn read_input(path: &str) -> Result<Vec<i32>, Error> {   
    let mut s = String::new();
    let file = File::open(path)?;
    let mut br = BufReader::new(file);
    br.read_line(&mut s).expect("read_line error");

    let parts: Vec<&str> = s.trim()
        .split(",")
        .collect();
    
    let mut nums: Vec<i32> = Vec::new();
    for part in parts {
        let n = part   
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; 
        nums.push(n);
    }

    Ok(nums)
}

fn initialize_memory(path: &String) -> Vec<i32> {
    let file_read_result = read_input(path);
    let memory: Vec<i32> = match file_read_result {
        Ok(m) => m,
        Err(error) => panic!("Problem parsing the masses: {:?}", error),
    };
    memory
}

fn initialize_instruction(memory: &Vec<i32>, addr: usize) -> Instruction {
    if memory[addr] == 99 {
        return Instruction {
            opcode: memory[addr],
            parameters: vec![],
        }
    } 
    Instruction {
        opcode: memory[addr],
        parameters: vec![
            memory[memory[addr+1] as usize], // first number
            memory[memory[addr+2] as usize], // second number
            memory[addr+3], // target address
        ],
    }
}


fn run_initcode(mut memory: Vec<i32>, start_addr: usize) -> Vec<i32> {
    let instr: Instruction = initialize_instruction(&memory, start_addr);
    if instr.opcode == 99 {
        return memory
    } else {
        match instr.opcode {
            1 => memory[instr.parameters[2] as usize] = instr.parameters[0] + instr.parameters[1],
            2 => memory[instr.parameters[2] as usize] = instr.parameters[0] * instr.parameters[1],
            _other => println!("Error! Invalid Op Code! {:?}", 
                memory[start_addr]),  
        };
        run_initcode(memory, start_addr+instr.size())
    }
}

fn main() {

    let test_initcode_before: Vec<i32> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let test_initcode_after = run_initcode(test_initcode_before, 0);
    assert_eq!(test_initcode_after[0], 3500);

    let test_initcode_before_2: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
    let test_initcode_after_2 = run_initcode(test_initcode_before_2, 0);
    assert_eq!(test_initcode_after_2[0], 30);

    let project_root_dir = match project_root::get_project_root() {
        Ok(p) => p,
        Err(e) =>  panic!("Error obtaining project root {:?}", e)
    };
    let input_data_file = format!("{}{}", 
        project_root_dir.into_os_string().into_string().unwrap(), 
        "/data/day_02.txt");

    /*
    Part 1:
    Once you have a working computer, the first step is to restore the 
    gravity assist program (your puzzle input) to the "1202 program alarm"
    state it had just before the last computer caught fire. To do this, 
    before running the program, replace position 1 with the value 12 and 
    replace position 2 with the value 2.
    */        
    let mut memory = initialize_memory(&input_data_file);
    memory[1] = 12;
    memory[2] = 2;
    let memory_after = run_initcode(memory, 0);
    println!("{:?}", &memory_after[0]);  // 5866663 */


    /*
    Part 2:
    Find the input noun and verb that cause the program to produce the 
    output 19690720. What is 100 * noun + verb? (For example, if 
    noun=12 and verb=2, the answer would be 1202.)
    */

    let memory = initialize_memory(&input_data_file);
    let vec: Vec<usize> = (1..memory.len()).collect();
    for perm in vec.iter().permutations(2).unique() {
        let mut memory_test = memory.clone();
        memory_test[1] = *perm[0] as i32;
        memory_test[2] = *perm[1] as i32;
        let memory_after = run_initcode(memory_test, 0);
        if memory_after[0] == 19690720 {
            println!("{:?}, {:?}", perm, 100*perm[0]+perm[1]); //[42, 59], 4259
            return 
        }
    }
}
