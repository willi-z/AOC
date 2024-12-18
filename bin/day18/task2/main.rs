use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

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

fn _calc_direction(from: &usize, to:&usize, max: &Coordinate) -> u8{
    let dx = usize::max(calc_index_x(from, max), calc_index_x(to, max)) -usize::min(calc_index_x(from, max), calc_index_x(to, max));
    //let dy = usize::max(calc_index_y(index_end, max), calc_index_y(index, max)) -usize::min(calc_index_y(index_end, max), calc_index_y(index, max));
    if dx != 0 {
        if calc_index_x(from, max) < calc_index_x(to, max) {
            return 1;
        } else {
            return 3;
        }
    } else {
        if calc_index_y(from, max) < calc_index_y(to, max) {
            return 2;
        } else {
            return 0;
        }
    }
}

struct Map{
    max: Coordinate,
    fields: Vec<char>
}

fn can_walk_to(index: &usize, direction: u8, map: &Map) -> bool {
    let index_next = next_index(index, direction, &map.max);
    if index_next == *index{
        return false
    }
    if map.fields[index_next] == '#'{
        return false;
    }
    return true
}

#[inline]
fn calc_cost_for_step(_direction_facing: u8, _direction_step: u8) -> usize{
    //let delta = u8::max(direction_facing, direction_step)- u8::min(direction_facing, direction_step);
    return 1;
}

fn calc_cost_heuristic(index: &usize, _direction: u8, index_end: &usize, max: &Coordinate)-> usize{
    let dx = usize::max(calc_index_x(index_end, max), calc_index_x(index, max)) -usize::min(calc_index_x(index_end, max), calc_index_x(index, max));
    let dy = usize::max(calc_index_y(index_end, max), calc_index_y(index, max)) -usize::min(calc_index_y(index_end, max), calc_index_y(index, max));
    return dx + dy;
}


fn reconstruct_path(came_from: &Vec::<usize>, index: usize) -> Vec<usize>{
    let mut path = Vec::new();
    path.push(index);
    let mut current = index;
    while came_from[current] != usize::MAX {
        current = came_from[current];
        path.insert(0, current);
    }
    return path;
}


fn astar_search(index_start: &usize, index_end: &usize, map: &Map) -> (Vec<usize>, usize) {
    let mut came_from = vec![usize::MAX; map.fields.len()];
    let mut g_scores = vec![usize::MAX; map.fields.len()];
    let mut f_scores = vec![usize::MAX; map.fields.len()];
    g_scores[*index_start] = 0;
    f_scores[*index_start] = calc_cost_heuristic(index_start, 1_u8, index_end, &map.max) + g_scores[*index_start];

    let mut stack = Vec::new();
    stack.push((*index_start,1_u8));
    
    while stack.len() != 0 {
        let (index, direction_current) = stack[0];
        if index == *index_end {
            return (reconstruct_path(&came_from, index), f_scores[index])
        }
        stack.remove(0);
        for direction in 0..4_u8 {
            //if direction == (direction_current+2)%4 {
            //    continue;
            //}
            if !can_walk_to(&index, direction, map){
                continue;
            }
            let g_score = g_scores[index] + calc_cost_for_step(direction_current, direction);
            let index_next = next_index(&index, direction, &map.max);
            if g_score < g_scores[index_next] {
                came_from[index_next] = index;
                g_scores[index_next] = g_score;
                f_scores[index_next] = calc_cost_heuristic(&index_next, direction, index_end, &map.max) + g_score;
                if !stack.iter().any(|(idx, dir)| *idx == index_next && *dir == direction) {
                    stack.push((index_next, direction));
                }
            }  
        }
        stack.sort_by(|&a,&b| f_scores[a.0].cmp(&f_scores[b.0]));
        //direction_current = calc_direction(&index, &stack[0], &map.max);
    }

   return (Vec::new(), usize::MAX)
}

fn _print_path(path:&Vec<usize>, map: &Map) -> Vec<char> {
    let mut map_with_path = map.fields.clone();
    for i in 1..path.len()-1{
        let from = path[i-1];
        let to = path[i];
        map_with_path[from]= 'O';
        map_with_path[to]= 'O';
    }
    let mut index = 0_usize;
    for _ in 0..map.max.y {
        for _ in 0..map.max.x {
            print!("{}", map_with_path[index]);
            index +=1;
        }
        println!()
    }
    return  map_with_path;
}

fn main() -> std::io::Result<()> {
    println!("Day 18: Task2");
    let is_debug = false;
    let file : File;
    let max: Coordinate;
    let max_bytes: usize;
    if is_debug {
        max = Coordinate{x:7, y:7};
        file = File::open("bin/day18/test.txt")?;
        max_bytes = 12;
    }
    else{
        file = File::open("bin/day18/input.txt")?;
        max = Coordinate{x:71, y:71};
        max_bytes = 1024;
    }
    let reader = BufReader::new(file);

    let index_start: usize = 0;
    let index_end: usize = max.x * max.y -1;

    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    
    let mut last_coord :String = String::new();
    let mut num_bytes= 0_usize;
    let mut map = Map{
        fields: vec!['.'; max.x * max.y],
        max: max,
    };
    for line in reader.lines() {
        let line = line?;
        last_coord = line.clone();
        if line.len() == 0 {
            continue;
        }
       
        // println!("{}",line);
        let cap = re.captures(&line).unwrap();
        
        let x: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let y: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        map.fields[y * map.max.x + x] = '#';
        num_bytes += 1;
        if num_bytes >= max_bytes{
           
        
            let(path, _cost) = astar_search(&index_start, &index_end,&map);
            if path.len() ==0 {
                break;
            }
        }
    }
    println!("{}", last_coord);
    Ok(())
}