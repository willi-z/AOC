use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main()  -> std::io::Result<()> {
    println!("Day 7: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day7/test.txt")?;
    }
    else{
        file = File::open("bin/day7/input.txt")?;
    }

    let reader = BufReader::new(file);
    let numbers = Regex::new(r"(\d+)").unwrap();
    let mut inputs: Vec<Vec<u64>> = Vec::new();
    for line in reader.lines(){
        let line = line?;
        let mut input : Vec<u64> = Vec::new();
        for (_, [num]) in numbers.captures_iter(&line).map(|c| c.extract()) {
            input.push(num.parse::<u64>().unwrap());
        }
        if input.len() == 0 {
            continue;
        }
        inputs.push(input);
    }

    let mut sum_correct: u64  = 0;
    for input in inputs {
        // println!("{:?}",input);
        if input.len() < 3 {
            continue;
        }
        let num_combination = (3usize).pow((input.len() -2).try_into().unwrap());
        for configuration in 0..num_combination {
            let mut result = input[1];
            for i in 2..input.len() {
                // 
                let op_id = (configuration / 3usize.pow((i-2) as u32)) % 3;
                //println!("{}|{}: {}", configuration, (i-2), op_id);
                match op_id {
                    0 => result += input[i],
                    1 => result *= input[i],
                    _ => result = result * 10u64.pow(((input[i] as f32).log10() as u32) + 1) + input[i],
                } 

                if result > input[0] {
                    break;
                }
            }
            if result == input[0] {
                sum_correct += result;
                break;
            }
        }
    }
    println!("{}",sum_correct);
    Ok(())
}