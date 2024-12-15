use std::{fs::File, io::{BufRead, BufReader}, vec};

struct Coordinate {
    x: usize,
    y: usize
}

#[inline]
fn calc_index_x(index: &usize, max: &Coordinate) -> usize {
    return index%max.x;
}

#[inline]
fn calc_index_y(index: &usize, max: &Coordinate) -> usize {
    return (index - (index%max.x))/max.x;
}

fn next_index(index: &usize, direction: u8, max: &Coordinate) -> usize {
    let mut new_index = index.clone();
    match direction {
        0 => if calc_index_y(index, max) != 0 {// top direction
            new_index -= max.x;
        },
        1 => if calc_index_x(index, max) < max.x -1 { // right direction
            new_index +=1;
        },
        2 => if calc_index_y(index, max) < max.y -1 { // down direction
            new_index += max.x;
        }
        _=> if calc_index_x(index, max) != 0 {
            new_index -=1;
        }
    }
    new_index
}

#[inline]
fn is_moveable_object(object_index: &usize, map: &Vec<u8>) -> bool {
    if map[*object_index] < 2 {
        return true;
    }
    return false;
}

#[inline]
fn is_empty(object_index: &usize, map: &Vec<u8>) -> bool {
    return map[*object_index] == 0;
}


fn move_object(object_index: &usize, direction: u8, map: &mut Vec<u8>, max: &Coordinate) -> bool{
    if !is_moveable_object(object_index, map){
        return false;
    }
    let next_index = next_index(object_index, direction, max);
    if next_index == *object_index {
        return false;
    }
    let mut moved = false;
    if is_moveable_object(&next_index, map) {
        moved = true;
        if !is_empty(&next_index, map) {
            moved = move_object(&next_index, direction, map, max)
        }
        if moved {
            // println!("moved {}|{}({}) to {}|{}({})", calc_index_x(object_index, max),calc_index_y(object_index, max),  map[*object_index],calc_index_x(&next_index, max),calc_index_y(&next_index, max),map[next_index]);
            map[next_index] = map[*object_index];
        }
    }
    return moved
}

fn _print_map(robot: usize, map: &Vec<u8>, max: &Coordinate) {
    let mut map_completed = vec!['.'; max.x * max.y];
    map_completed[robot] = '@';
    for (index, entry) in map.iter().enumerate() {
        if *entry == 1 {
            map_completed[index] = 'O';
        } else if *entry == 2 {
            map_completed[index] = '#';
        }
    }
    let mut index = 0_usize;
    for _ in 0..max.y{
        for _ in 0..max.x{
            //print!("{}", map_completed[index]);
            index += 1
        }
        println!()
    }
}


fn main() -> std::io::Result<()> {
    println!("Day 15: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day15/test.txt")?;
    }
    else{
        file = File::open("bin/day15/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map: Vec<u8> = Vec::new();
    let mut robot = 0;
    let mut directions: Vec<u8> = Vec::new();
    
    let mut read_map = true;
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            read_map = false;
        }
        if read_map {
            max_x = line.len();
            for field in line.chars(){
                match field {
                    '@' => {
                        robot = map.len();
                        map.push(0);
                    }
                    'O' => map.push(1),
                    '#' => map.push(2),
                    _=> map.push(0)
                }
            }
            max_y += 1;
        } else {
            for field in line.chars(){
                match field {
                    '^' => {
                        directions.push(0);
                    }
                    '>' => directions.push(1),
                    'v' => directions.push(2),
                    '<' => directions.push(3),
                    _ => println!("Warning: {}", field)
                }
            }
        }
        
    }
    let max = Coordinate{x: max_x, y: max_y};

    for direction in directions{
        // println!("direction: {direction}");
        if move_object(&robot, direction, &mut map, &max) {
            robot = next_index(&robot, direction, &max);
        }
    }
    //_print_map(robot, &map, &max);
    
    let mut result = 0_usize;
    for (index, value) in map.iter().enumerate(){
        if *value != 1 {
            continue;
        }
        result += 100* calc_index_y(&index, &max) + calc_index_x(&index, &max);
    }

    println!("Result: {:?}", result);
    return Ok(());
}