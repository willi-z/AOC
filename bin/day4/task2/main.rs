use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main()  -> std::io::Result<()> {
    println!("Day 4: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day4/task2/test.txt")?;
    }
    else{
        file = File::open("bin/day4/input.txt")?;
    }

    let mut puzzel: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    let re_forward = Regex::new(r"MAS").unwrap();
    let re_backward = Regex::new(r"SAM").unwrap();
    

    // check horizontal
    for line in reader.lines() { 
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        
        puzzel.push(line);
    }
    let mut found_mas = 0;
    let n_rows = puzzel.len();
    let n_cols = puzzel[0].len();
    for row in 0..(n_rows-2) {
        for col in 0..(n_cols-2) {
            let chars = vec![
                puzzel[row].chars().nth(col).unwrap(), 
                puzzel[row+1].chars().nth(col+1).unwrap(),
                puzzel[row+2].chars().nth(col+2).unwrap()
            ];
            let diagonal: String = chars.into_iter().collect();
            let found_mas_0 = re_forward.find(&diagonal).is_some() || re_backward.find(&diagonal).is_some() ;
            
            let chars = vec![
                puzzel[row].chars().nth(col+2).unwrap(), 
                puzzel[row+1].chars().nth(col+1).unwrap(),
                puzzel[row+2].chars().nth(col).unwrap()
            ];
            let diagonal: String = chars.into_iter().collect();
            let found_mas_1 = re_forward.find(&diagonal).is_some() || re_backward.find(&diagonal).is_some() ;
            if found_mas_0 && found_mas_1 {
                found_mas += 1
            }
        }
    }

    println!("{}",found_mas);
    Ok(())
}