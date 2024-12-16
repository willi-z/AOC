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

fn calc_direction(from: &usize, to:&usize, max: &Coordinate) -> u8{
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
fn calc_cost_for_step(direction_facing: u8, direction_step: u8) -> usize{
    let delta = u8::max(direction_facing, direction_step)- u8::min(direction_facing, direction_step);
    let rotations = u8::min(delta, 4-delta);
    return 1000_usize * rotations as usize+ 1;
}

fn calc_cost_heuristic(index: &usize, direction: u8, index_end: &usize, max: &Coordinate)-> usize{
    let dx = usize::max(calc_index_x(index_end, max), calc_index_x(index, max)) -usize::min(calc_index_x(index_end, max), calc_index_x(index, max));
    let dy = usize::max(calc_index_y(index_end, max), calc_index_y(index, max)) -usize::min(calc_index_y(index_end, max), calc_index_y(index, max));
    let mut rotations = 0_usize;
    if dx != 0{
        if calc_index_x(index_end, max) >  calc_index_x(index, max){
            if direction != 1{
                rotations+=1;
                if direction == 3{
                    rotations+=1;
                }
            }
        } else {
            if direction != 3{
                rotations+=1;
                if direction == 1{
                    rotations+=1;
                }
            }
        }
    }
    if dy != 0{
        if calc_index_y(index_end, max) >  calc_index_y(index, max){
            if direction != 2{
                rotations+=1;
                if direction == 0{
                    rotations+=1;
                }
            }
        } else {
            if direction != 0{
                rotations+=1;
                if direction == 2{
                    rotations+=1;
                }
            }
        }
    }
    return 1000 *rotations + dx + dy;
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

fn _print_path(path:&Vec<usize>, map: &Map) {
    let mut map_with_path = map.fields.clone();
    for i in 1..path.len()-1{
        let from = path[i-1];
        let to = path[i];
        map_with_path[to] = match calc_direction(&from, &to, &map.max) {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => '?'
        }
    }
    let mut index = 0_usize;
    for _ in 0..map.max.y {
        for _ in 0..map.max.x {
            print!("{}", map_with_path[index]);
            index +=1;
        }
        println!()
    }
}

fn main() -> std::io::Result<()> {
    println!("Day 16: Task1");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day16/test1.txt")?;
    }
    else{
        file = File::open("bin/day16/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut max_x = 0;
    let mut max_y = 0;

    let mut fields: Vec<char> = Vec::new();
    let mut index_start: usize = 0;
    let mut index_end: usize = 0;
    
    let mut index = 0_usize;
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        max_x = line.len();
        for field in line.chars(){
            match field {
                'S' => {
                    index_start = index;
                    fields.push(field);
                },
                'E' => {
                    index_end = index;
                    fields.push(field);
                },
                '.' => fields.push(field),
                '#' => fields.push(field),
                _=> println!()
            }
            index += 1;
        }
        max_y += 1;
    }
    let map = Map{
        max: Coordinate{x: max_x, y: max_y},
        fields: fields,
    };

    let(path, cost) = astar_search(&index_start, &index_end,&map);
    _print_path(&path, &map);
    println!();
    println!("{}:{:?}", cost, path);

    let mut cost= 0;
    let mut direction_facing = 1_u8;
    for i in 1..path.len(){
        let direction_step = calc_direction(&path[i-1], &path[i], &map.max);
        cost += calc_cost_for_step(direction_facing, direction_step);
        direction_facing = direction_step;
    }
    println!("TESTED: {}", cost);
    Ok(())
}