use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[inline]
fn has_even_digits(num: usize) -> bool{
    return ((num as f64).log10() as usize) %2 != 0;
}



fn num_of_stones(stone: usize, remaining_cycles: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize{
    if remaining_cycles == 0 {
        return 1;
    }
    let cached_result = cache.get(&(stone, remaining_cycles));
    if cached_result.is_some(){
        return cached_result.unwrap().clone();
    } else {
        let result: usize;
        if stone == 0 {
            result = num_of_stones(1, remaining_cycles-1, cache);
        } else if has_even_digits(stone) {
            let divider = 10_usize.pow(((stone as f64).log10().floor() as u32 +1) / 2);
            result = num_of_stones(stone / divider, remaining_cycles-1, cache) + num_of_stones(stone % divider, remaining_cycles-1, cache)
        } else {
            result = num_of_stones(stone * 2024, remaining_cycles-1, cache);
        }
        cache.insert((stone, remaining_cycles), result);
        return  result;
    }
}

fn main() -> std::io::Result<()>{
    println!("Day11: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day11/test.txt")?;
    } else {
        file = File::open("bin/day11/input.txt")?;
    }
    let mut stones :Vec<usize>= Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line?;
        for val in line.split_whitespace() {
            stones.push(val.parse().unwrap())
        }
    }
    println!("{:?}",stones);
    let num_of_blinks = 75;
    let mut cache = HashMap::new();
    let mut result = 0;
    for stone in stones {
        result += num_of_stones(stone, num_of_blinks, &mut cache)
    }
    println!("{}", result);
    Ok(())
}