use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    x: usize,
    y: usize
}

fn find_next_obstacle_in_loop(pos_index: usize, collision_dir: u8, max_x: &mut usize, max_y: &mut usize, obstacle_positions: &Vec<Position>) -> usize{
    let pos = &obstacle_positions[pos_index];
    let mut index_next = obstacle_positions.len();
    let dir = (collision_dir + 1) % 4;
    if dir == 0 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
           if obstacle_pos.x == pos.x +1 && obstacle_pos.y < pos.y && obstacle_pos.y + *max_y >= pos.y {
               *max_y = pos.y - obstacle_pos.y;
               index_next = index;
           }
        }
    } else if dir == 1 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.y == pos.y +1 && pos.x < obstacle_pos.x  && pos.x + *max_x >= obstacle_pos.x {
                *max_x = obstacle_pos.x - pos.x;
                index_next = index;
            }
        }
    } else if dir == 2 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.x +1 == pos.x && pos.y < obstacle_pos.y && pos.y + *max_y >= obstacle_pos.y {
                *max_y = obstacle_pos.y - pos.y;
                index_next = index;
            }
        }
    } else {
            for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.y +1 == pos.y && obstacle_pos.x < pos.x && obstacle_pos.x + *max_x >= pos.x {
                *max_x = pos.x - obstacle_pos.x;
                index_next = index;
            }
        }
    }
    index_next
}

fn find_previous_obstacle_in_loop(pos_index: usize, collision_dir: u8, max_x: &mut usize, max_y: &mut usize, obstacle_positions: &Vec<Position>) -> usize{
    let pos = &obstacle_positions[pos_index];
    if collision_dir == 0 {
        for obstacle_pos in obstacle_positions {
            if pos.x == obstacle_pos.x && obstacle_pos.y > pos.y && pos.y + *max_y >= obstacle_pos.y {
                *max_y = obstacle_pos.y - pos.y;
            }
        }
    } else if collision_dir == 1 {
        for obstacle_pos in obstacle_positions {
            if pos.y == obstacle_pos.y && pos.x > obstacle_pos.x && obstacle_pos.x + *max_x >= pos.x {
                *max_x = pos.x - obstacle_pos.x;
            }
        }
    } else if collision_dir == 2 {
        for obstacle_pos in obstacle_positions {
            if pos.x == obstacle_pos.x && pos.y > obstacle_pos.y && obstacle_pos.y + *max_y >= pos.y {
                *max_y = pos.y - obstacle_pos.y;
            }
        }
    } else  {
        for obstacle_pos in obstacle_positions {
            if pos.y == obstacle_pos.y && obstacle_pos.x > pos.x && pos.x + *max_x >= obstacle_pos.x {
                *max_x = obstacle_pos.x - pos.x;
            }
        }
    }
    let mut index_prev = obstacle_positions.len();
    let dir = (collision_dir + 3) % 4;
    if dir == 1 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.x == pos.x +1 && obstacle_pos.y < pos.y && obstacle_pos.y + *max_y >= pos.y {
                *max_y = pos.y - obstacle_pos.y;
                index_prev = index;
            }
        }
    } else if dir == 2 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.y == pos.y +1 && pos.x < obstacle_pos.x && pos.x + *max_x >= obstacle_pos.x {
                *max_x = obstacle_pos.x - pos.x;
                index_prev = index;
            }
        }
    } else if dir == 3 {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.x +1 == pos.x && pos.y < obstacle_pos.y && pos.y+ *max_y >= obstacle_pos.y {
                *max_y = obstacle_pos.y - pos.y;
                index_prev = index;
            }
        }
    } else {
        for (index, obstacle_pos) in obstacle_positions.iter().enumerate() {
            if obstacle_pos.y +1 == pos.y && obstacle_pos.x < pos.x && obstacle_pos.x + *max_x >= pos.x {
                *max_x = pos.x - obstacle_pos.x;
                index_prev = index;
            }
        }
    }
    index_prev
}


fn check_opposite_paths(corner_pos: &Position, collision_dir: u8, delta: & Position, obstacle_positions: &Vec<Position>) -> bool{
    let mut is_clear = true;
    let paths: Position;
    match collision_dir {
        0 => {
            paths = Position{x: corner_pos.x + delta.x - 1, y: corner_pos.y + delta.y};
            for obst in obstacle_positions {
                if (corner_pos.x <= obst.x && obst.x <= paths.x && paths.y == obst.y) || (obst.x == paths.x && corner_pos.y < obst.y && obst.y <= paths.y){
                    is_clear = false;
                    break;
                } 
            }
        }
        1 => {
            paths = Position{x: corner_pos.x - delta.x, y: corner_pos.y + delta.y -1};
            for obst in obstacle_positions {
                // println!("check: {:?}", obst);
                if (corner_pos.x > obst.x && obst.x >= paths.x && paths.y == obst.y) || (obst.x == paths.x && corner_pos.y <= obst.y && obst.y <= paths.y){
                    // println!("collision!");
                    is_clear = false;
                    break;
                } 
            }
        }
        2 => {
            paths = Position{x: corner_pos.x - delta.x +1, y: corner_pos.y - delta.y};
            for obst in obstacle_positions {
                if (corner_pos.x >= obst.x && obst.x >= paths.x && paths.y == obst.y) || (obst.x == paths.x && corner_pos.y > obst.y && obst.y >= paths.y){
                    is_clear = false;
                    break;
                } 
            }
        }
        _ => {
            paths = Position{x: corner_pos.x + delta.x, y: corner_pos.y - delta.y +1 };
            for obst in obstacle_positions {
                if (corner_pos.x < obst.x && obst.x <= paths.x && paths.y == obst.y) || (obst.x == paths.x && corner_pos.y >= obst.y && obst.y >= paths.y){
                    is_clear = false;
                    break;
                } 
            }
        }
    }
    return is_clear;
}

fn calc_diag_position(pos: &Position, collision_dir: u8, delta: &Position, max: &Position) -> Option<Position>{
    //println!("calc_diag_position: {:?}, {}", pos, collision_dir);
    match collision_dir {
        0 => if pos.y + delta.y + 1 <= max.y {
                Some(Position{x: pos.x + delta.x - 1, y: pos.y + delta.y + 1})
            } else {
                None
            },
        1 => if pos.x - delta.x >= 1 {
                Some(Position{x: pos.x - delta.x -1, y: pos.y + delta.y -1})
            } else {
                None
            },
        2 => if pos.y - delta.y  >= 1{
                Some(Position{x: pos.x - delta.x +1, y: pos.y - delta.y -1})
            } else {
                None
            },
        _ => if pos.x + delta.x +1 <= max.x {
                Some(Position{x: pos.x + delta.x +1, y: pos.y - delta.y +1 })
            } else {
                None
            }
    }
}

fn main()  -> std::io::Result<()> {
    println!("Day 6: Task2");
    let is_debug = true;
    let file : File;
    if is_debug {
        file = File::open("bin/day6/test.txt")?;
    }
    else{
        file = File::open("bin/day6/input.txt")?;
    }

    let reader = BufReader::new(file);
    
    let mut guard : (Position, u8) = (Position { x: 0, y: 0 }, 0);

    
    let mut obstacle_ids: HashMap<Position, usize> = HashMap::new();
    let mut obstacle_positions: Vec<Position> = Vec::new();
    
    let mut size_x = 0;
    let mut size_y = 0;
    for (y_pos, line) in reader.lines().enumerate() {
        let line = line?;
        for (x_pos, character) in line.chars().enumerate(){
            let pos = Position { x: x_pos, y: y_pos };
            if character == '#' {
                obstacle_ids.insert(pos.clone(), obstacle_positions.len());
                obstacle_positions.push(pos);
            } else if character == '^' {
                guard = (pos, 0);
            }
            size_x = x_pos;
        }
        size_y = y_pos;
    }

    //println!("activated: {:?}", obstacle_activated);
    // sim
    let mut obstacle_activated = [Vec::<usize>::new(), Vec::<usize>::new(), Vec::<usize>::new(), Vec::<usize>::new()];
    loop {
        // activate obstacles to the left (guard perspektive)
        let mut pos = guard.0.clone();
        let dir_left = (guard.1 + 3) % 4;
        if dir_left == 0 {
            if pos.y == 0 {
                break;
            }
            pos.y -= 1;
        } else if dir_left == 1 {
            if pos.x == size_x{
                break;
            }
            pos.x += 1;
        } else if dir_left == 2 {
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
        let obst = obstacle_ids.get(&pos);
        if obst.is_some(){
            obstacle_activated[dir_left as usize].push(*obst.unwrap());
        }

        // check direct collision
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
        let obst = obstacle_ids.get(&pos);
        if obst.is_some(){
            //obstacle_activated[guard.1 as usize].push(*obst.unwrap());
            guard.1 = (guard.1 + 1) % 4;
            continue;
        }
        guard.0 = pos;
    }
    //println!("activated: {:?}", obstacle_activated);

    println!("dims: {}|{}", size_x, size_y);
    let mut obstacle_new :HashSet<Position>= HashSet::new();
    for (collision_dir, candiadates) in obstacle_activated.iter().enumerate() {
        for seed_obstacle_id in candiadates {
            let mut max_x = size_x.clone();
            let mut max_y = size_y.clone();

            
            let index_next = find_next_obstacle_in_loop(*seed_obstacle_id, collision_dir as u8, &mut max_x, &mut max_y, &obstacle_positions);
            let index_prev = find_previous_obstacle_in_loop(*seed_obstacle_id, collision_dir as u8, &mut max_x, &mut max_y, &obstacle_positions);

            println!("{seed_obstacle_id}: neigh: {}|{} (max: {})", index_prev, index_next, obstacle_positions.len());
            if index_next == obstacle_positions.len() && index_prev == obstacle_positions.len() {
                break
            }
            if index_next != obstacle_positions.len() && index_prev != obstacle_positions.len() {
                println!("{:?}: neigh: prev: {:?}| next: {:?}", &obstacle_positions[*seed_obstacle_id], &obstacle_positions[index_prev], &obstacle_positions[index_next]);
                let pos = calc_diag_position(&obstacle_positions[*seed_obstacle_id], collision_dir as u8,&Position { x: max_x, y: max_y }, &Position { x: size_x, y: size_y });
                if pos.is_none() {
                    continue;
                }
                let pos = pos.unwrap();
                if obstacle_ids.get(&pos).is_some(){
                    println!("exists already!");
                    continue;
                }

                if !check_opposite_paths(&obstacle_positions[*seed_obstacle_id], collision_dir as u8,&Position { x: max_x, y: max_y }, &obstacle_positions) {
                    println!("other obstacles on path!");
                    continue;
                }

                println!("both: {:?}|{} (delta: {:?}) => {:?}", &obstacle_positions[*seed_obstacle_id], collision_dir, Position { x: max_x, y: max_y }, &pos);
                obstacle_new.insert(pos);
            }

            if index_prev == obstacle_positions.len() {
                print!("no prev: ");
                let index_diag = find_next_obstacle_in_loop(index_next, (collision_dir as u8 +1)%4 , &mut max_x, &mut max_y, &obstacle_positions);
                if index_diag == obstacle_positions.len() {
                    println!("no found!");
                    continue;
                }
                let pos = calc_diag_position(&obstacle_positions[index_next], (collision_dir as u8 +1)%4 ,&Position { x: max_x, y: max_y }, &Position { x: size_x, y: size_y });
                if pos.is_none() {
                    continue;
                }
                let pos = pos.unwrap();
                print!("{:?}|{} (delta: {:?}) => {:?} ", &obstacle_positions[index_next], (collision_dir as u8 +1)%4 , Position { x: max_x, y: max_y }, &pos);
                if obstacle_ids.get(&pos).is_some(){
                    println!("exists!");
                    continue;
                }
                if !check_opposite_paths(&obstacle_positions[index_next], (collision_dir as u8 +1)%4 ,&Position { x: max_x, y: max_y }, &obstacle_positions) {
                    println!("other obstacles on path!");
                    continue;
                }
                println!("ok!");
                obstacle_new.insert(pos);
            } else{
                let index_diag = find_previous_obstacle_in_loop(index_prev, (collision_dir as u8 +3)%4, &mut max_x, &mut max_y, &obstacle_positions);
                if index_diag == obstacle_positions.len() {
                    continue;
                }
                let pos =calc_diag_position(&obstacle_positions[index_prev], (collision_dir as u8 +3)%4,&Position { x: max_x, y: max_y }, &Position { x: size_x, y: size_y });
                if pos.is_none() {
                    continue;
                }
                let pos = pos.unwrap();
                println!("no next: {:?}|{} (delta: {:?}) => {:?}", &obstacle_positions[index_prev], (collision_dir as u8 +3)%4, Position { x: max_x, y: max_y }, &pos);
                if obstacle_ids.get(&pos).is_some(){
                    continue;
                }
                if !check_opposite_paths(&obstacle_positions[index_prev], (collision_dir as u8 +3)%4,&Position { x: max_x, y: max_y }, &obstacle_positions) {
                    println!("other obstacles on path!");
                    continue;
                }
                obstacle_new.insert(pos);
            }
        }
    }
    // 3,6
    // 6,7
    // 7,7
    // 1,8
    // 3,8
    // 7,9
    println!("{:?}", obstacle_new);
    println!("{}", obstacle_new.len());
    Ok(())
}