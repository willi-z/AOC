use std::fs::File;
use std::io::{BufRead, BufReader};


fn main()  -> std::io::Result<()> {
    println!("Day 1: Task1");
    let file = File::open("bin/day1/input.txt")?;
    let reader = BufReader::new(file); 
    let mut col1: Vec<i32> = Vec::new(); 
    let mut col2: Vec<i32> = Vec::new(); 
    for line in reader.lines() { 
        let line = line?; 
        let mut values = line.split_whitespace(); 
        if let (Some(val1), Some(val2)) = (values.next(), values.next()) { 
            if let (Ok(num1), Ok(num2)) = (val1.parse::<i32>(), val2.parse::<i32>()) { 
                col1.push(num1); col2.push(num2); 
            } 
        } 
    }
    assert_eq!(col1.len(), col2.len());
    col1.sort();
    col2.sort();
    
    let result: i32 = col1.iter().zip(col2.iter()).map(|(a, b)| (a - b).abs()).collect::<Vec<_>>().into_iter().sum();
    println!("Distance: {}",result);
    Ok(())
}
