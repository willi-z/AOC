use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};


fn main()  -> std::io::Result<()> {
    println!("Day 3: Task2");
    let file = File::open("bin/day3/input.txt")?;
    let reader = BufReader::new(file); 
    let re_markers = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re_numbers = Regex::new(r"(\d{1,3}),(\d{1,3})").unwrap();
    let mut results = vec![];
    let mut is_enabled = true;
    for line in reader.lines() {
        for matches in re_markers.find_iter(&line.unwrap()).map(|m| m.as_str()) {
            match matches {
                "do()" => is_enabled = true,
                "don't()" => is_enabled = false,
                _ => if is_enabled {
                    for (_, [in1, in2]) in re_numbers.captures_iter(matches).map(|c| c.extract()) {
                        //println!("{},{}",in1, in2);
                        results.push(in1.parse::<u32>().unwrap() * in2.parse::<u32>().unwrap());
                    }
                }
            }
            
        }
    }
    println!("{:?}",results);
    println!("{:?}",results.iter().sum::<u32>());
    Ok(())
}