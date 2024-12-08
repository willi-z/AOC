use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: usize,
    y: usize
}

fn end_with_loop(mut guard: (Position, u8), obstacles: HashSet<Position>, dims: &Position) -> bool{
    let mut is_loop = false;
    let mut seen_obstacles = HashSet::new();
    loop {
        let mut pos = guard.0.clone();
        if guard.1 == 0 {
            if pos.y == 0 {
                break;
            }
            pos.y -= 1;
        } else if guard.1 == 1 {
            if pos.x == dims.x -1{
                break;
            }
            pos.x += 1;
        } else if guard.1 == 2 {
            if pos.y == dims.y -1{
                break;
            }
            pos.y += 1;
        } else {
            if pos.x == 0 {
                break;
            }
            pos.x -= 1;
        }
        if obstacles.contains(&pos){
            if seen_obstacles.contains(&(pos.x, pos.y, guard.1)) {
                is_loop = true;
                break;
            }else {
                seen_obstacles.insert((pos.x, pos.y, guard.1));
            }
            guard.1 = (guard.1 + 1) % 4;
            continue;
        }
        guard.0 = pos;
    }
    is_loop
}

fn main()  -> std::io::Result<()> {
    println!("Day 6: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day6/test3.txt")?;
    }
    else{
        file = File::open("bin/day6/input.txt")?;
    }

    let reader = BufReader::new(file);
    
    let mut guard_start : (Position, u8) = (Position { x: 0, y: 0 }, 0);

    let mut obstacle_positions = HashSet::new();
    let mut invalid_positions = HashSet::new();
    
    let mut size_x = 0;
    let mut size_y = 0;
    for (y_pos, line) in reader.lines().enumerate() {
        let line = line?;
        for (x_pos, character) in line.chars().enumerate(){
            let pos = Position { x: x_pos, y: y_pos };
            if character == '#' {
                invalid_positions.insert(pos.clone());
                obstacle_positions.insert(pos);
            } else if character == '^' {
                invalid_positions.insert(pos.clone());
                guard_start = (pos, 0);
            }
            size_x = x_pos;
        }
        size_y = y_pos;
    }

    let dims = Position{x: size_x +1, y: size_y+1};

    //println!("activated: {:?}", obstacle_activated);
    // sim
    let mut obstacles_new_loop = HashSet::new();
    for i in 0..dims.y {
        for j in 0.. dims.x {
            if invalid_positions.contains(&Position{x:j, y:i}) {
                continue
            }
            let mut obstacles = obstacle_positions.clone();
            obstacles.insert(Position{x:j, y:i});
            if end_with_loop(guard_start.clone(), obstacles, &dims){
                obstacles_new_loop.insert(Position{x:j, y:i});
            }
        }
    }
    if is_debug {

    } else {
        assert_eq!(obstacles_new_loop.len(), 1721);
    }

    // 3,6
    // 6,7
    // 7,7
    // 1,8
    // 3,8
    // 7,9
    println!("{:?}", obstacles_new_loop);
    println!("{}", obstacles_new_loop.len());
    Ok(())
}