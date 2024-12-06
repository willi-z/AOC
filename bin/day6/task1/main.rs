use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::vec;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize
}

fn main()  -> std::io::Result<()> {
    println!("Day 6: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day6/test.txt")?;
    }
    else{
        file = File::open("bin/day6/input.txt")?;
    }

    let reader = BufReader::new(file);
    
    let mut guard : (Position, u8) = (Position { x: 0, y: 0 }, 0);
    let mut obstacles: HashMap<Position, u8> = HashMap::new();
    
    let mut size_x = 0;
    let mut size_y = 0;
    for (y_pos, line) in reader.lines().enumerate() {
        let line = line?;
        for (x_pos, character) in line.chars().enumerate(){
            if character == '#' {
                obstacles.insert(Position { x: x_pos, y: y_pos }, 1);
            } else if character == '^' {
                guard = (Position { x: x_pos, y: y_pos }, 0);
            }
            size_x = x_pos;
        }
        size_y = y_pos;
    }

    let mut map = vec![vec![0u8;size_x+1];size_y+1];

    loop {
        map[guard.0.y][guard.0.x] = 1;
        let mut pos = guard.0.clone();
        if guard.1 == 0 {
            if pos.y == 0 {
                break;
            }
            pos.y -= 1;
        } else if guard.1 == 1 {
            if pos.x == size_x{
                break;
            }
            pos.x += 1;
        } else if guard.1 == 2 {
            if pos.y == size_y{
                break;
            }
            pos.y += 1;
        } else {
            if pos.x == 0 {
                break;
            }
            pos.x -= 1;
        }
        let obst = obstacles.get(&pos);
        if obst.is_some(){
            guard.1 = (guard.1 + 1) % 4;
            continue;
        }
        guard.0 = pos;
    }
    println!("{:?}", map);
    let sum = map.iter().map(|vec| vec.iter().filter(|&&num| num == 1).count()).sum::<usize>();

    println!("{}", sum);
    Ok(())
}