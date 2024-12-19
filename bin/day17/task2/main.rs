use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;


struct Computer {
    instruction_pointer: u64,
    register_a: u64,
    register_b: u64,
    register_c: u64,
}
impl Computer {
    fn process(&mut self, program: &Vec<u8>) -> Vec<u8> {
        let mut output = Vec::new();
        //println!("{:?}", program);
        while (self.instruction_pointer as usize) < program.len() - 1{
            let op_code = program[self.instruction_pointer as usize];
            let input = program[(self.instruction_pointer +1) as usize] as u64;
            let combo_code = get_combo_code(input, self.register_a, self.register_b, self.register_c);
            
            self.register_a = (op_code != 0) as u64 * self.register_a + (op_code == 0) as u64 * (self.register_a/ 2_u64.pow((op_code == 0) as u32 * combo_code as u32));
            self.register_b = ((op_code != 1) && (op_code != 2) && (op_code != 4) && (op_code != 6)) as u64 * self.register_b + (op_code == 1) as u64 * (self.register_b ^ input) + (op_code == 2) as u64 * combo_code%8 + (op_code == 4) as u64 * (self.register_b ^ self.register_c) + (op_code == 6) as u64 * (self.register_a/ 2_u64.pow((op_code == 6) as u32 *combo_code as u32));
            self.register_c = (op_code != 7) as u64 * self.register_c + (op_code == 7) as u64 * (self.register_a/ 2_u64.pow((op_code == 7) as u32 *combo_code as u32));
            if op_code == 5{
                output.push((combo_code%8) as u8);
            }
            self.instruction_pointer = (op_code!= 3 || self.register_a == 0) as u64 * (self.instruction_pointer +2) + (op_code == 3 && self.register_a != 0) as u64 * input;
        }
        //println!("R: {:?}", output);
        return output
    }

    fn fit_self(&mut self, program: &Vec<u8>) -> Vec<u8> {
        let mut output = Vec::new();
        while (self.instruction_pointer as usize) < program.len() - 1{
            let op_code = program[self.instruction_pointer as usize];
            let input = program[(self.instruction_pointer +1) as usize] as u64;
            let combo_code = get_combo_code(input, self.register_a, self.register_b, self.register_c);
            
            self.register_a = (op_code != 0) as u64 * self.register_a + (op_code == 0) as u64 * (self.register_a/ 2_u64.pow((op_code == 0) as u32 * combo_code as u32));
            self.register_b = ((op_code != 1) && (op_code != 2) && (op_code != 4) && (op_code != 6)) as u64 * self.register_b + (op_code == 1) as u64 * (self.register_b ^ input) + (op_code == 2) as u64 * combo_code%8 + (op_code == 4) as u64 * (self.register_b ^ self.register_c) + (op_code == 6) as u64 * (self.register_a/ 2_u64.pow((op_code == 6) as u32 *combo_code as u32));
            self.register_c = (op_code != 7) as u64 * self.register_c + (op_code == 7) as u64 * (self.register_a/ 2_u64.pow((op_code == 7) as u32 *combo_code as u32));
            if op_code == 5{
                output.push((combo_code%8) as u8);
                if *output.last().unwrap() != program[output.len()-1] {
                    break;
                }
            }
            self.instruction_pointer = (op_code!= 3 || self.register_a == 0) as u64 * (self.instruction_pointer +2) + (op_code == 3 && self.register_a != 0) as u64 * input;
        }
        return output
    }
}

#[inline]
fn get_combo_code(input: u64, register_a: u64, register_b: u64, register_c: u64) -> u64{
    return  (input<4) as u64 * input + (input==4) as u64 * register_a + (input==5) as u64 * register_b + (input==6) as u64 * register_c
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test0() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 0,
            register_c: 9,
        };
        
        let program = vec![2,6];

        computer.process(&program);

        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn test1() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 29,
            register_c: 0,
        };
        
        let program = vec![1,7];

        computer.process(&program);
        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test3() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            
        };
        
        let program = vec![4,0];
        computer.process(&program);

        assert_eq!(computer.register_b, 44354);
    }

    #[test]
    fn test4() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 46187030,
            register_b: 0,
            register_c: 0,
            
        };
        
        let program = vec![2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0];
        let output = computer.process(&program);
        let expected: Vec<u8> = vec![7,1,3,4,1,2,6,7,1];

        assert_eq!(output, expected);
    }

    #[test]
    fn test5() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 117440,
            register_b: 0,
            register_c: 0,
            
        };
        
        let program = vec![0,3,5,4,3,0];
        let output = computer.process(&program);

        assert_eq!(output, program);
    }


}

fn main() -> std::io::Result<()> {
    println!("Day 17: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day17/test2.txt")?;
    }
    else{
        file = File::open("bin/day17/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut code = String::new();
    for line in reader.lines(){
        let line = line?;
        code.push_str(&line);
    }

    let re_a = Regex::new(r"Register A: (\d+)").unwrap();
    let re_b = Regex::new(r"Register B: (\d+)").unwrap();
    let re_c = Regex::new(r"Register C: (\d+)").unwrap();

    let re_prog = Regex::new(r"(Program: .+$)").unwrap();
    let re_prog1 = Regex::new(r"(\d)").unwrap();
    
    let cap = re_a.captures(&code).unwrap();
    let register_a = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();

    let cap = re_b.captures(&code).unwrap();
    let register_b = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();

    let cap = re_c.captures(&code).unwrap();
    let register_c = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();

    let cap = re_prog.captures(&code).unwrap();
    let code = cap.get(1).unwrap().as_str();

    let mut program = Vec::new();
    for (_, [val]) in re_prog1.captures_iter(&code).map(|c| c.extract()) {
        program.push(val.parse::<u8>().unwrap());
    }

    println!("A: {register_a}");
    println!("B: {register_b}");
    println!("C: {register_c}");
    println!();
    println!("P: {:?}", program);

    let mut canidates = vec![0_u64];
    
    let mut computer: Computer;
    for i in 0..program.len() {
        let num = program[program.len()- i-1];
        //println!("Num: {}", num);
        let mut new_canidates = Vec::new();
        //println!("Candidates: {:?}", canidates);
        for candidate in canidates.iter(){
            let canidate = *candidate <<3;
            for register_a in canidate..canidate+8 {
                computer = Computer{
                    instruction_pointer: 0,
                    register_a: register_a,
                    register_b: register_b,
                    register_c: register_c,
                };
            
                let output = computer.process(&program);
                if *output.first().unwrap() == num {
                    //println!("Out: {:?}", output);
                    new_canidates.push(register_a);
                    //break;
                }
            }
        }
        canidates.clear(); 
        for candidate in new_canidates {
            canidates.push(candidate)
        }
    }
    
    println!("Register A: {:?}", canidates[0]);
    computer = Computer{
        instruction_pointer: 0,
        register_a: canidates[0],
        register_b: register_b,
        register_c: register_c,
    };

    let output = computer.process(&program);
    println!("{:?}", output);
    
    Ok(())
}