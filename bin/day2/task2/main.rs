use std::fs::File;
use std::io::{BufRead,BufReader};


fn check_safety(levels: &[i32] ) -> bool {
    let has_correct_distance = levels.windows(2).map(|e| (e[1] - e[0]).abs()).all(|num| num >= 1 && num <= 3);
    let directions: Vec<i32> = levels.windows(2).map(|e| (e[1] - e[0]).signum()).collect();
    let all_ones = directions.iter().all(|e| *e == 1);
    let all_neg_ones = directions.iter().all(|e| *e == -1);
    let is_monton = all_ones || all_neg_ones;
    has_correct_distance && is_monton
}

fn main()  -> std::io::Result<()> {
    println!("Day 2: Task2");
    let is_debug = false;
    let file : File;
    if !is_debug {
        file= File::open("bin/day2/input.txt")?;
    }else {
        file = File::open("bin/day2/task2/test.txt")?;
    }
    let reader = BufReader::new(file); 

    let mut found_safe = 0u32;
    for line in reader.lines() { 
        let line = line?;
        if is_debug {
            println!("{}",line);
        }
        let levels: Vec<i32> = line.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect();
        if levels.len() == 0 {
            continue;
        }
        if check_safety(&levels) {
            found_safe += 1;
            if is_debug {
                println!("SAFE");
            }
            continue;
        }
  
        let distances: Vec<i32> = levels.windows(2).map(|e| (e[1] - e[0]).abs()).collect();
        let correct_distances: Vec<bool> = distances.iter().map(|&num| num >= 1 && num <= 3).collect();
        let ids_false_distances: Vec<usize> = correct_distances.iter().enumerate().filter_map(|(index, &value)| if !value { Some(index) } else { None }).collect();
        if ids_false_distances.len() == 1  || ids_false_distances.len() == 2 {
            for remove_index in ids_false_distances.iter() {
                let mut new_levels: Vec<i32> = Vec::new();
                for (index, &value) in levels.iter().enumerate() { 
                    if *remove_index != index { 
                        new_levels.push(value); 
                    } 
                }
                if check_safety(&new_levels) {
                    found_safe += 1;
                    if is_debug {
                        println!("SAFE: diff 1");
                    }
                    break;
                }
                let mut new_levels: Vec<i32> = Vec::new();
                for (index, &value) in levels.iter().enumerate() { 
                    if *remove_index + 1 != index { 
                        new_levels.push(value); 
                    } 
                }
                if check_safety(&new_levels) {
                    found_safe += 1;
                    if is_debug {
                        println!("SAFE: diff 2");
                    }
                    break;
                }
            }
            continue;
        }
        let directions: Vec<i32> = levels.windows(2).map(|e| (e[1] - e[0]).signum()).collect();
        let num_ascending = directions.iter().filter(|&&e| e == 1).count();
        let ids_false_ascending: Vec<usize>;
        if num_ascending > directions.len()/2{
            ids_false_ascending = directions.iter().enumerate().filter_map(|(index, &value)| if value < 1 { Some(index) } else { None }).collect();
        }else {
            ids_false_ascending = directions.iter().enumerate().filter_map(|(index, &value)| if value > -1 { Some(index) } else { None }).collect();
        }
        if ids_false_ascending.len() == 1 {
            for remove_index in ids_false_ascending.iter() {
                let mut new_levels: Vec<i32> = Vec::new();
                for (index, &value) in levels.iter().enumerate() { 
                    if *remove_index != index { 
                        new_levels.push(value); 
                    } 
                }
                if check_safety(&new_levels) {
                    found_safe += 1;
                    if is_debug {
                        println!("SAFE: asc 1");
                    }
                    break;
                }
                let mut new_levels: Vec<i32> = Vec::new();
                for (index, &value) in levels.iter().enumerate() { 
                    if *remove_index + 1 != index { 
                        new_levels.push(value); 
                    } 
                }
                if check_safety(&new_levels) {
                    found_safe += 1;
                    if is_debug {
                        println!("SAFE: asc 2");
                    }
                    break;
                }
            }
            continue;
        } 
    }
    if !is_debug {
        assert_ne!(found_safe, 807);
        assert_ne!(found_safe, 689);
        assert_ne!(found_safe, 736);
        assert_ne!(found_safe, 670);
    }
    
    
    println!("No. of Safes: {}",found_safe);
    Ok(())
}