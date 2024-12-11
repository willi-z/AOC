use std::fs::File;
use std::io::{BufRead, BufReader};


#[inline]
fn has_even_digits(num: usize) -> bool{
    return ((num as f64).log10() as usize) %2 != 0;
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
    let num_of_blinks = 25;
    for _ in 0..num_of_blinks{
        let mut stones_new = Vec::with_capacity(stones.len() *2);
        for stone in stones {
            if stone == 0 {
                stones_new.push(1);
            } else if has_even_digits(stone) {
                let divider = 10_usize.pow(((stone as f64).log10().floor() as u32 +1) / 2);
                stones_new.push(stone / divider);
                stones_new.push(stone % divider);
            } else {
                stones_new.push(stone * 2024);
            }
        }
        stones = stones_new;
        //println!("{:?}",stones);
    }
    println!("{}", stones.len());
    Ok(())
}