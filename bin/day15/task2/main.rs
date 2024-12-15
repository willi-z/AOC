use std::{fs::File, io::{BufRead, BufReader}};

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
fn is_moveable_object(object_index: &usize, map: &Vec<char>) -> bool {
    return map[*object_index] != '#';
}

#[inline]
fn is_empty(object_index: &usize, map: &Vec<char>) -> bool {
    return map[*object_index] == '.';
}

fn can_object_move(object_index: &usize, direction: u8, map: &Vec<char>, max: &Coordinate) -> bool{
    if !is_moveable_object(object_index, map){
        return false;
    }
    let index_this: usize;
    if direction%2 == 0 {
        if map[*object_index] == ']'{
            index_this = *object_index -1;
        }
        else {
            index_this = *object_index
        }
    } else {
        index_this = *object_index
    }
    let next_index0 = next_index(&index_this, direction, max);
    if next_index0 == index_this {
        return false;
    }
    let mut next_index1 = next_index0;
    if direction%2 == 0 {
        if map[index_this] == '[' {
            next_index1 = next_index(&(index_this+1), direction, max);
        }
    }
    let nexts: Vec<usize>;
    if next_index0 == next_index1{
        nexts = vec![next_index0];
    } else {
        nexts = vec![next_index0, next_index1]
    }

    let mut is_moveable = true;
    for index in nexts{
        if is_moveable_object(&index, map) {
            if !is_empty(&index, map) {
                if !can_object_move(&index, direction, map, max) {
                    is_moveable = false;
                    break
                }
            }
        } else{
            is_moveable = false;
            break
        }
    }
    //println!("{}|{}: {}", calc_index_x(&index_this, max), calc_index_y(&index_this, max), is_moveable);
    return is_moveable;
}

fn move_object(object_index: &usize, direction: u8, map: &mut Vec<char>, max: &Coordinate) -> bool {
    if map[*object_index] == '@' {
        if !can_object_move(object_index, direction, map, max){
            return false;
        }
    } else {
        if !is_moveable_object(object_index, map){
            return false
        }
        if is_empty(object_index, map) {
            return true
        }
    }
    let index_this: usize;
    if direction%2 == 0 {
        if map[*object_index] == ']'{
            index_this = *object_index -1;
        }
        else {
            index_this = *object_index
        }
    } else {
        index_this = *object_index
    }

    let next_index0 = next_index(&index_this, direction, max);
    if next_index0 == index_this {
        return false;
    }
    let mut next_index1 = next_index0;
    if direction%2 == 0 {
        if map[index_this] == '[' {
            next_index1 = next_index(&(index_this+1), direction, max);
        }
    }
    let nexts: Vec<usize>;
    if next_index0 == next_index1{
        nexts = vec![next_index0];
    } else {
        nexts = vec![next_index0, next_index1]
    }

    for (offset,index) in nexts.iter().enumerate(){
        if move_object(index, direction, map, max) {
            map[*index] = map[index_this+offset];
            map[index_this+offset] = '.'
        }
    }
    return true

}


fn _print_map(map: &Vec<char>, max: &Coordinate) {
    let mut index = 0_usize;
    for _ in 0..max.y{
        for _ in 0..max.x{
            print!("{}", map[index]);
            index += 1
        }
        println!()
    }
}


fn main() -> std::io::Result<()> {
    println!("Day 15: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day15/test1.txt")?;
    }
    else{
        file = File::open("bin/day15/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map: Vec<char> = Vec::new();
    let mut robot = 0;
    let mut directions: Vec<u8> = Vec::new();
    
    let mut read_map = true;
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            read_map = false;
        }
        if read_map {
            max_x = line.len()*2;
            for field in line.chars(){
                match field {
                    '@' => {
                        robot = map.len();
                        map.push('@');
                        map.push('.');
                    }
                    'O' => {
                        map.push('[');
                        map.push(']');
                    },
                    '#' => {
                        map.push('#');
                        map.push('#');
                    },
                    '.' => {
                        map.push('.');
                        map.push('.');
                    }
                    _ =>println!("Warning: {}", field)
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
    _print_map(&map, &max);

    for direction in directions{
        if is_debug{
            println!("direction: {direction}");
        }
        
        if move_object(&mut robot, direction, &mut map, &max){
            robot = next_index(&robot, direction, &max);
        }
        if is_debug{
            _print_map(&map, &max);
        }
    }
    _print_map(&map, &max);
    
    let mut result = 0_usize;
    for (index, value) in map.iter().enumerate(){
        if *value != '[' {
            continue;
        }
        let x =  calc_index_x(&index, &max);
        let y =  calc_index_y(&index, &max);
        result += 100* y + x;
    }

    if is_debug{
        assert_eq!(result, 9021);
    } else {
        assert_ne!(result, 1436321);
    }

    println!("Result: {:?}", result);
    return Ok(());
}