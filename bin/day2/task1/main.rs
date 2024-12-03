use std::fs::File;
use std::io::{BufRead,BufReader};


fn main()  -> std::io::Result<()> {
    println!("Day 2: Task1");
    let file = File::open("bin/day2/input.txt")?;
    let reader = BufReader::new(file); 

    let mut found_safe = 0u32;
    for line in reader.lines() { 
        let line = line?; 
        let levels: Vec<i32> = line.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect();
        let has_correct_distance = levels.windows(2).map(|e| (e[1] - e[0]).abs()).all(|num| num >= 1 && num <= 3);
        let directions: Vec<i32> = levels.windows(2).map(|e| (e[1] - e[0]).signum()).collect();
        let all_ones = directions.iter().all(|e| *e == 1);
        let all_neg_ones = directions.iter().all(|e| *e == -1);
        let is_monton = all_ones || all_neg_ones;
        let is_safe = has_correct_distance && is_monton;
        if is_safe {
            found_safe += 1
        }
        println!("{:?}",levels)
    }
    
    println!("No. of Safes: {}",found_safe);
    Ok(())
}