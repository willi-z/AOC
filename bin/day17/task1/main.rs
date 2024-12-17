use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;


type LiteralOperation = fn(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize;

type Operation = fn(input: usize, state: &mut Computer);

struct Computer {
    instruction_pointer: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    op_codes: Vec<Operation>,
    combo_codes: Vec<LiteralOperation>,
    output: Vec<usize>
}
impl Computer {
    fn process(&mut self, program: &Vec<u8>) -> bool {
        if self.instruction_pointer >= program.len() {
            println!("aborted");
            return false
        }
        let op_code_id = program[self.instruction_pointer] as usize;
        let op_code = self.op_codes[op_code_id];

        op_code(program[self.instruction_pointer +1] as usize, self);
        //println!("{op_code_id} & {lit_code_id}-> {lit_code_result}");
        // println!("B: {}", self.register_b);
        return true
    }
}

fn combo0(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return 0;
}

fn combo1(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return 1;
}

fn combo2(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return 2;
}

fn combo3(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return 3;
}

fn combo4(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return *_register_a;
}

fn combo5(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return *_register_b;
}

fn combo6(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    return *_register_c;
}

fn combo7(_register_a: &usize, _register_b: &usize, _register_c: &usize) -> usize {
    println!("ERROR");
    return 0;
}

fn adv(input: usize, state: &mut Computer) {
    //println!("adv");
    let combo_result = state.combo_codes[input](&state.register_a, &state.register_b, &state.register_c);
    state.register_a = (state.register_a as f64 / 2_usize.pow(combo_result as u32) as f64).trunc() as usize;
    state.instruction_pointer += 2;
}

fn bxl(input: usize, state: &mut Computer) {
    //println!("bxl");
    state.register_b ^= input;
    state.instruction_pointer += 2;
}

fn bst(input: usize, state: &mut Computer) {
    // println!("bst: {lit_op_result}");
    let combo_result = state.combo_codes[input](&state.register_a, &state.register_b, &state.register_c);
    state.register_b = combo_result%8;
    state.instruction_pointer += 2;
}

fn jnz(input: usize, state: &mut Computer) {
    //println!("jnz");
    if state.register_a == 0 {
        state.instruction_pointer += 2;
        return
    }
    state.instruction_pointer = input;
}

fn bxc(_input: usize, state: &mut Computer) {
    //println!("bxc");
    state.register_b ^= state.register_c;
    state.instruction_pointer += 2;
}

fn out(input: usize, state: &mut Computer) {
    //println!("out");
    let combo_result = state.combo_codes[input](&state.register_a, &state.register_b, &state.register_c);
    state.output.push(combo_result%8);
    state.instruction_pointer += 2;
}

fn bdv(input: usize, state: &mut Computer) {
    //println!("bdv");
    let combo_result = state.combo_codes[input](&state.register_a, &state.register_b, &state.register_c);
    state.register_b = (state.register_a as f64 / 2_usize.pow(combo_result as u32) as f64).trunc() as usize;
    state.instruction_pointer += 2;
}

fn cdv(input: usize, state: &mut Computer) {
    //println!("cdv");
    let combo_result = state.combo_codes[input](&state.register_a, &state.register_b, &state.register_c);
    state.register_c = (state.register_a as f64 / 2_usize.pow(combo_result as u32) as f64).trunc() as usize;
    state.instruction_pointer += 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 0,
            register_c: 9,
            output: Vec::new(),
            combo_codes: vec![combo0, combo1, combo2, combo3, combo4, combo5, combo6, combo7],
            op_codes: vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv]
        };
        
        let program = vec![2,6];

        while computer.process(&program){

        }

        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn test1() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 29,
            register_c: 0,
            output: Vec::new(),
            combo_codes: vec![combo0, combo1, combo2, combo3, combo4, combo5, combo6, combo7],
            op_codes: vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv]
        };
        
        let program = vec![1,7];

        while computer.process(&program){

        }

        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test3() {
        let mut computer = Computer{
            instruction_pointer: 0,
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            output: Vec::new(),
            combo_codes: vec![combo0, combo1, combo2, combo3, combo4, combo5, combo6, combo7],
            op_codes: vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv]
        };
        
        let program = vec![4,0];

        while computer.process(&program){

        }

        assert_eq!(computer.register_b, 44354);
    }


}

fn main() -> std::io::Result<()> {
    println!("Day 17: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day17/test.txt")?;
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
    let register_a = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let cap = re_b.captures(&code).unwrap();
    let register_b = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let cap = re_c.captures(&code).unwrap();
    let register_c = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();

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

    
    let mut computer = Computer{
        instruction_pointer: 0,
        register_a: register_a,
        register_b: register_b,
        register_c: register_c,
        output: Vec::new(),
        combo_codes: vec![combo0, combo1, combo2, combo3, combo4, combo5, combo6, combo7],
        op_codes: vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv]
    };

    loop {
        if !computer.process(&program){
            break;
        }
    }
    
    println!("{:?}",computer.output);
    println!("{:?}", computer.output.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(","));
    Ok(())
}