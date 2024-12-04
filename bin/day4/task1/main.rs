use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main()  -> std::io::Result<()> {
    println!("Day 4: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day4/task1/test.txt")?;
    }
    else{
        file = File::open("bin/day4/input.txt")?;
    }

    let mut puzzel: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    let re_forward = Regex::new(r"XMAS").unwrap();
    let re_backward = Regex::new(r"SAMX").unwrap();
    let mut found_xmas = 0;

    // check horizontal
    for line in reader.lines() { 
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        found_xmas += re_forward.find_iter(&line).count();
        found_xmas += re_backward.find_iter(&line).count();
        puzzel.push(line);
    }
    println!("after horizontal check:{}", found_xmas);

    // check vertical
    let mut puzzel_90: Vec<String> = vec![String::new(); puzzel[0].len()]; 
    for row in puzzel.iter() { 
        for (i, val) in row.chars().enumerate() { 
            puzzel_90[i].push(val); 
        } 
    }
    for line in puzzel_90.iter() {
        found_xmas += re_forward.find_iter(&line).count();
        found_xmas += re_backward.find_iter(&line).count();
    }
    println!("after vertical check:{}", found_xmas);

    
    
    let n_cols = puzzel[0].len();
    let n_rows = puzzel.len();
    // check diagonal top left to bottom right
    let mut puzzel_diagonal: Vec<String> = Vec::new();
    for col in 0..n_cols { 
        let mut diagonal = String::new(); 
        let mut i = 0; 
        let mut j = col; 
        while i < n_rows && j < n_cols { 
            if let Some(c) = puzzel[i].chars().nth(j) {
                diagonal.push(c); 
            } else{
                break;
            }
            i += 1; 
            j += 1; 
        } 
        puzzel_diagonal.push(diagonal);
    }
    for row in 1..n_rows { 
        let mut diagonal = String::new(); 
        let mut i = row; 
        let mut j = 0; 
        while i < n_rows && j < n_cols { 
            if let Some(c) = puzzel[i].chars().nth(j) {
                diagonal.push(c); 
            } else{
                break;
            }
            i += 1; j += 1; 
        } 
        puzzel_diagonal.push(diagonal); 
    }
    for line in puzzel_diagonal.iter() {
        found_xmas += re_forward.find_iter(&line).count();
        found_xmas += re_backward.find_iter(&line).count();
    }
    println!("after diagonal left-right check:{}", found_xmas);

    // check diagonal top right to bottom left
    let mut puzzel_diagonal: Vec<String> = Vec::new();
    for col in 0..(n_cols+1) { 
        let mut diagonal = String::new(); 
        let mut i = 0; 
        let mut j = col; 
        while i < n_rows && j > 0 { 
            if let Some(c) = puzzel[i].chars().nth(j-1) {
                diagonal.push(c); 
            } else{
                break;
            }
            i += 1; 
            j -= 1; 
        } 
        puzzel_diagonal.push(diagonal);
    }
    for row in 1..n_rows { 
        let mut diagonal = String::new(); 
        let mut i = row; 
        let mut j = n_cols; 
        while i < n_rows && j > 0 { 
            if let Some(c) = puzzel[i].chars().nth(j-1) {
                diagonal.push(c); 
            } else{
                break;
            } 
            i += 1; 
            j -= 1; 
        } 
        puzzel_diagonal.push(diagonal); 
    }
    for line in puzzel_diagonal.iter() {
        found_xmas += re_forward.find_iter(&line).count();
        found_xmas += re_backward.find_iter(&line).count();
    }
    if is_debug {
        assert_eq!(found_xmas, 18);
    } else {
        assert_ne!(found_xmas, 2625);
        assert_eq!(found_xmas, 2633);
    }

    println!("{}",found_xmas);
    Ok(())
}