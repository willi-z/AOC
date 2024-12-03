use std::fs::File;
use std::io::{BufRead,BufReader};


fn main()  -> std::io::Result<()> {
    println!("Day 2: Task1");
    let file = File::open("bin/day2/task1/input.txt")?;
    let reader = BufReader::new(file); 

    let mut found_safe = 0u32;
    for line in reader.lines() { 
        let line = line?; 
        let mut values = line.split_whitespace();
        let mut last_number = 0i32;
        let mut is_safe = true;
        if let Some(value) = values.next() {
            if let Ok(num) = value.parse::<i32>(){
                last_number = num;
            }
            else {
                is_safe = false;
            }
        }
        
        let mut current_number = 0i32;
        if let Some(value) = values.next() {
            if let Ok(num) = value.parse::<i32>(){
                current_number = num;
            }
        }
        let mut difference = current_number - last_number;
        
        last_number = current_number;

        for value in values {
            if let Ok(num) = value.parse::<i32>(){
                current_number = num;
            }
            let diff = current_number - last_number;
            if diff.signum() != difference.signum() {
                is_safe = false;
                break;
            }
            difference = diff;
            if diff.abs() == 0 || diff.abs() > 3i32{
                is_safe = false;
                break;
            }
            last_number = current_number;
        }
        if is_safe {
            found_safe += 1;
            println!("{}: {}",line, is_safe)
        }
        println!("{}",is_safe)
    }
    
    println!("No. of Safes: {}",found_safe);
    Ok(())
}