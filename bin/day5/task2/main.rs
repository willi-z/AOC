use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main()  -> std::io::Result<()> {
    println!("Day 5: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day5/task1/test.txt")?;
    }
    else{
        file = File::open("bin/day5/input.txt")?;
    }

    let reader = BufReader::new(file);
    let re_rules = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut rules: Vec<(u32,u32)> = Vec::new();
    let mut line_iter = reader.lines();
    while let Some(line) = line_iter.next() {
        let line = line?;
        let mut found_rule = false;
        for (_, [in1, in2]) in re_rules.captures_iter(&line).map(|val|val.extract()){
            rules.push((in1.parse().unwrap(), in2.parse().unwrap()));
            found_rule = true;
        }
        if !found_rule {
            break;
        }
    }

    let mut input_incorect :Vec<Vec<u32>> = Vec::new();
    while let Some(line) = line_iter.next() {
        let line = line?;
        let mut input: Vec<u32> = Vec::new();
        let mut input_map : HashMap<u32, usize>= HashMap::new();
        for (index, val) in line.split(',').enumerate() {
            let num = val.parse::<u32>().unwrap();
            input.push(num);
            input_map.insert(num, index);
        }
        if input.len() == 0 {
            continue;
        }
        // println!("{:?}", input);
        let mut is_correct = true;
        for (num0, num1) in rules.iter() {
            let index0  = input_map.get(&num0);
            if index0.is_none(){
                continue;
            }
            let index1  = input_map.get(&num1);
            if index1.is_none(){
                continue;
            }
            if index0.unwrap() > index1.unwrap() {
                is_correct = false;
                break;
            }
        }
        if !is_correct {
            input_incorect.push(input);
        }

    }

    let mut sum = 0u32;
    for input in input_incorect {
        let c
        println!("{}",input[(input.len()-1)/2]);
        sum += input[(input.len()-1)/2];
    }
    println!("{}", sum);
    Ok(())
}