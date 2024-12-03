use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main()  -> std::io::Result<()> {
    println!("Day 3: Task1");
    let file = File::open("bin/day3/input.txt")?;
    let reader = BufReader::new(file); 
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = vec![];
    for line in reader.lines() { 
        for (_, [in1, in2]) in re.captures_iter(&line.unwrap()).map(|c| c.extract()) {
            results.push(in1.parse::<u32>().unwrap()* in2.parse::<u32>().unwrap());
        }
    }
    println!("{:?}",results.iter().sum::<u32>());
    Ok(())
}