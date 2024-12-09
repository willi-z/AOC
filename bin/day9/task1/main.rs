use std::{fs::File, io::{BufRead, BufReader}};

struct Block {
    position: usize,
    id: usize,
}


fn main() -> std::io::Result<()> {
    println!("Day 9: Task1");
    //println!("size: {}", std::mem::size_of::<(u8,u8)>());
    let is_debug = true;
    let file : File;
    if is_debug {
        file = File::open("bin/day9/test.txt")?;
    }
    else{
        file = File::open("bin/day9/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut memory = Vec::new();
    let mut position = 0;
    let mut index = 0;
    for line in reader.lines(){
        let line = line?;
        for (marker, length) in line.chars().enumerate() {
            let length = length.to_digit(10).unwrap();
            if marker % 2 == 0{
                for _ in 0..length{
                    memory.push(Block{position: position, id: index});
                    position += 1;
                }
                index += 1;
            } else {
                position += length as usize;
            }
        }
    }

    let mut idx_start = 0;
    let mut idx_end = memory.len();
    let mut memory_compressed = vec![0; memory.len()];
    for i in 0..memory_compressed.len() {
        if idx_start == idx_end {
            break
        }
        if i == memory[idx_start].position{
            memory_compressed[i] = memory[idx_start].id;
            idx_start += 1;
        }
        else {
            idx_end -= 1;
            memory_compressed[i] =memory[idx_end].id;
        }
    }

    let mut checksum = 0;
    for (pos, id) in memory_compressed.iter().enumerate(){
        checksum += pos*id;
    }
    println!("{}", checksum);
    Ok(())
}