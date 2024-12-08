use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: i64,
    y: i64
}

fn is_valid(pos: &Position, max: &Position) -> bool{
    if pos.x <0 || pos.y <0 {
        return false;
    }
    if pos.x >= max.x || pos.y >= max.y {
        return false;
    }
    true
}

fn main()  -> std::io::Result<()> {
    println!("Day 8: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day8/test.txt")?;
    }
    else{
        file = File::open("bin/day8/input.txt")?;
    }

    let reader = BufReader::new(file);

    
    let mut antenas: HashMap<char, Vec::<Position>> = HashMap::new();
    
    let mut size_x = 0;
    let mut size_y = 0;
    for (y_pos, line) in reader.lines().enumerate() {
        let line = line?;
        for (x_pos, character) in line.chars().enumerate(){
            size_x = x_pos;
            if character == '.' {
                continue
            }
            let pos = Position { x: x_pos as i64, y: y_pos as i64};
            let positions = antenas.get_mut(&character);
            if positions.is_some() {
                positions.unwrap().push(pos);
            } else{
                antenas.insert(character, vec![pos]);
            }
        }
        size_y = y_pos;
    }
    size_x += 1;
    size_y += 1;

    let max: Position = Position{x:size_x as i64, y: size_y as i64};
   

    let mut antinodes_positions = HashSet::new();
    for (_, positions) in antenas {
        if positions.len() == 1 {
            continue;
        }
        for i in 0 ..positions.len()-1{
            let pos_0 = &positions[i];
            for j in i+1..positions.len(){
                let pos_1 = &positions[j];
                let dist = Position{x: (pos_1.x - pos_0.x), y: (pos_1.y - pos_0.y)};
                let pos_a = Position{x: pos_1.x + dist.x, y: pos_1.y + dist.y};
                let pos_b = Position{x: pos_0.x - dist.x, y: pos_0.y - dist.y};
                //println!("{:?} + {:?}: {:?} && {:?}", pos_0, pos_1, pos_a, pos_b);
                if is_valid(&pos_a, &max){
                    antinodes_positions.insert(pos_a);
                }
                if is_valid(&pos_b, &max){
                    antinodes_positions.insert(pos_b);
                }
            }
        }
    }
    println!("{}",antinodes_positions.len());
    Ok(())
}