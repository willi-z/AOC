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
        let mut indices = HashMap::new();
        let mut previous = Vec::new();
        //let mut afters = Vec::new();
        for (index, num) in input.iter().enumerate() {
            indices.insert(*num, index);
            previous.push(Vec::<u32>::new());
            //afters.push(Vec::<u32>::new());
        }
        for rule in &rules {
            let deps  = (indices.get(&rule.0), indices.get(&rule.1));
            if deps.0.is_none() || deps.1.is_none() {
                continue;
            }
            previous[*deps.1.unwrap()].push(rule.0);
            //afters[*deps.0.unwrap()].push(rule.1);
        }
        // println!("{:?}", previous);
        //println!("{:?}", afters);
        let mut input_sorted = Vec::new();

        let mut is_imperfect = true;
        while is_imperfect {
            is_imperfect = false;
            for num in input.iter(){
                if input_sorted.contains(num) {
                    continue;
                }
                let prev = &mut previous[*indices.get(num).unwrap()];
                let mut remove_idx = Vec::new();
                for index in 0..prev.len(){
                    if input_sorted.contains(&prev[index]){
                        remove_idx.push(index);
                    }
                }
                for i in 0..remove_idx.len() {
                    prev.remove(remove_idx[remove_idx.len() - i -1]);
                }
                //println!("{}: {:?}", num, prev);
                if prev.len() == 0 {
                    input_sorted.push(*num);
                }
                else{
                    is_imperfect = true
                }
            }
        }
        //print!("{:?}: ", input_sorted);
        //println!("{}",input_sorted[(input_sorted.len()-1)/2]);
        sum += input_sorted[(input_sorted.len()-1)/2];
    }
    println!("{}", sum);
    Ok(())
}