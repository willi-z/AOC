use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;


fn main()  -> std::io::Result<()> {
    println!("Day 1: Task2");
    let file = File::open("bin/day1/input.txt")?;
    let reader = BufReader::new(file); 
    let mut list_left: Vec<i32> = Vec::new(); 
    let mut list_right = HashMap::new();
    for line in reader.lines() { 
        let line = line?; 
        let mut values = line.split_whitespace(); 
        if let (Some(val1), Some(val2)) = (values.next(), values.next()) { 
            if let (Ok(num1), Ok(num2)) = (val1.parse::<i32>(), val2.parse::<i32>()) { 
                list_left.push(num1);
                *list_right.entry(num2).or_insert(0) += 1
            } 
        } 
    }
    let mut result = 0;
    for num in list_left.iter(){
        result += num * match list_right.get(&num) {
            Some(founds) => founds,
            None => &0
        }
    }
    println!("Distance: {}",result);
    Ok(())
}
