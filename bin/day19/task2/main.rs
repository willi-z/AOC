use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

fn solve(input: String, towels: &HashSet<String>, cache: &mut HashMap<String, usize>) -> usize{
    //print!("{}| ", input);
    if !cache.contains_key(&input){
        if input.len() == 0{
            return 1;
        }
        else{
            let mut counter = 0_usize;
            for towel in towels{
                if input.starts_with(towel){
                    counter += solve(input.split_at(towel.len()).1.to_string(), towels, cache);
                }
            }
            cache.insert(input, counter);
            return counter
        }
    } else {
        return *cache.get(&input).unwrap();
    }
    
}

fn main() -> std::io::Result<()> {
    println!("Day 19: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day19/test.txt")?;
    }
    else{
        file = File::open("bin/day19/input.txt")?;
    }
    let reader= BufReader::new(file);
    let mut lines = reader.lines();

    let mut towels= HashSet::new();
    if let Some(line) = lines.next(){
        let line = line?;
        for towel in line.split(", "){
            towels.insert(towel.to_string());
        }
    }

    let _ = lines.next();

    let mut counter = 0_usize;
    let mut cache = HashMap::new();
    for line in lines { 
        let design = line?;
        //println!("{}, {}", &design, solve(design.clone(), &towels, &mut cache));
        counter += solve(design, &towels, &mut cache);
    }
    println!("{}", counter);
    Ok(())
}