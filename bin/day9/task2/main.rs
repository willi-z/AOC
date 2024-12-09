use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Clone, Debug)]
struct Block {
    position: usize,
    id: u16,
    length: u8
}


fn main() -> std::io::Result<()> {
    println!("Day 9: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day9/test.txt")?;
    }
    else{
        file = File::open("bin/day9/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut memory = Vec::new();
    let mut memory_holes: Vec<_> = Vec::new();
    let mut position = 0;
    let mut index = 0;
    for line in reader.lines(){
        let line = line?;
        for (marker, length) in line.chars().enumerate() {
            let length = length.to_digit(10).unwrap() as u8;
            if marker % 2 == 0{
                memory.push(Block{position: position, id: index, length: length});
                position += length as usize;
                index += 1;
            } else {
                memory_holes.push(Block{position: position, id: 0, length: length});
                position += length as usize;
            }
        }
    }
    

    //let mut count_end = 0;
    //let mut position = 0;
    let mut memory_compressed :Vec<Block> = Vec::with_capacity(memory.len());
    for hole in memory_holes {
        let mut position = hole.position;
        let mut remove_from_memory = Vec::new();
        for (index,block) in memory.iter().enumerate() {
            if block.position < hole.position {
                memory_compressed.push(block.clone());
                remove_from_memory.push(index);
            }
        }

        for i in 0..remove_from_memory.len() {
            memory.remove(remove_from_memory[remove_from_memory.len()-1-i]);
        }

        let mut remove_from_memory = Vec::new();
        let mut space = hole.length;
        for i in 0..memory.len() {
            let idx_end = memory.len()-1 -i;
            if memory[idx_end].length <= space {
                let mut block = memory[idx_end].clone();
                //println!("INSERT found: {:?}", block_candidates[block_idx]);
                block.position = position;
                position += block.length as usize;
                space -= block.length;
                memory_compressed.push(block);
                remove_from_memory.push(idx_end);
                // println!("NEW space: {}", space);
            }
        }

        for i in 0..remove_from_memory.len() {
            memory.remove(remove_from_memory[i]);
        }
    }
    println!("");
    //println!("{:?}", memory_compressed);

    let mut checksum = 0;
    for block in memory_compressed.iter(){
        checksum += ((2*block.position + (block.length as usize) -1) * block.length as usize)/2 *(block.id as usize);
    }
    println!("{}", checksum);
    Ok(())
}