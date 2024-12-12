use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
struct Side {
    index: usize,
    normal_direction: usize,
    length_right: usize
}

fn is_part_of_side(index: &usize, side: &Side, max: &Coordinate) -> bool{
    match side.normal_direction {
        0 => {
            if calc_index_y(index, max) == calc_index_y(&side.index, max) && calc_index_x(&side.index, max) <= calc_index_x(index, max) && calc_index_x(&side.index, max) + side.length_right >= calc_index_x(index, max) {
                return true
            }
        },
        1 => {
            if calc_index_x(index, max) == calc_index_x(&side.index, max) && calc_index_y(&side.index, max) <= calc_index_y(index, max) && calc_index_y(&side.index, max) + side.length_right >= calc_index_y(index, max){
                return true
            }
        },
        2 => {
            if calc_index_y(index, max) == calc_index_y(&side.index, max) && calc_index_x(index, max) <= calc_index_x(&side.index, max) && calc_index_x(index, max) + side.length_right >= calc_index_x(&side.index, max) {
                return true
            }
        },
        _ => {
            if calc_index_x(index, max) == calc_index_x(&side.index, max) && calc_index_y(index, max) <= calc_index_y(&side.index, max) && calc_index_y(index, max) + side.length_right >= calc_index_y(&side.index, max) {
                return true
            }
        }
    }
    return false
}

fn extend_side_left(side: &mut Side, map: &Vec<char>, max: &Coordinate) -> bool {
    let next_left = next_index(&side.index, &((side.normal_direction+3)%4), max);
    if next_left != side.index {
        if map[next_left] == map[side.index] {
            if has_normal(&next_left, &side.normal_direction, map, max){
                side.index = next_left;
                return true;
            }
        }
    }
    return false;
}

fn extend_side_right(side: &mut Side, map: &Vec<char>, max: &Coordinate) -> bool {
    let next_right = next_index_with_offset(&side.index, &((side.normal_direction+1)%4), &side.length_right, max);
    if next_right != side.index {
        if map[next_right] == map[side.index] {
            if has_normal(&next_right, &side.normal_direction, map, max){
                side.length_right += 1;
                return true;
            }
        }
    }
    return false;
}

fn calc_index_x(index: &usize, max: &Coordinate) -> usize {
    return index%max.x;
}

fn calc_index_y(index: &usize, max: &Coordinate) -> usize {
    return (index - (index%max.x))/max.x;
}

fn next_index(index: &usize, direction: &usize, max: &Coordinate) -> usize {
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

fn next_index_with_offset(index: &usize, direction: &usize, offset: &usize,  max: &Coordinate) -> usize{
    let mut new_index = index.clone();
    match direction {
        0 => if calc_index_y(index, max) >= *offset {// top direction
            new_index -= max.x * offset;
        },
        1 => if calc_index_x(index, max) + offset < max.x { // right direction
            new_index += offset;
        },
        2 => if calc_index_y(index, max) + offset < max.y { // down direction
            new_index += max.x * offset;
        }
        _=> if calc_index_x(index, max) >= *offset {
            new_index -= offset;
        }
    }
    new_index
}

fn has_normal(index: &usize, direction: &usize, map: &Vec<char>, max: &Coordinate) -> bool{
    return next_index(index, direction, max) == *index || map[next_index(index, direction, max)] != map[*index]
}

fn group(index: &usize, map: &Vec<char>, max: &Coordinate, group_id: &mut usize, groups: &mut HashMap<usize, usize>) {
    groups.insert(*index, *group_id);
    for direction in 0..4{
        let next_index = next_index(&index, &direction, &max);
        if next_index == *index || map[*index] != map[next_index]{
            continue;
        }
        let id =  groups.get(&next_index);
        if id.is_none(){
            group(&next_index, map, max, group_id, groups);
        } 
    }
}


fn main() -> std::io::Result<()> {
    println!("Day 12: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day12/test.txt")?;
    }
    else{
        file = File::open("bin/day12/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = Vec::new();
    
    
    for line in reader.lines() {
        let line = line?;
        max_x = line.len();
        for plant in line.chars(){
            map.push(plant);
        }
        max_y += 1;
    }
    let max = Coordinate{x: max_x, y: max_y};

    let mut groups: HashMap<usize, usize> = HashMap::new();
    let mut group_id = 0;
    for index in 0..map.len(){ // map.len()
        if groups.get(&index).is_none(){
            group(&index, &map, &max, &mut group_id, &mut groups);
            group_id += 1;
        }
    }

    let mut region_areas: Vec<usize> = vec![0; group_id];
    for index in 0..map.len(){ 
        let group_id = groups.get(&index).unwrap();
        region_areas[*group_id] += 1;
    }
    println!("areas: {:?}", region_areas);

    let mut region_sides: Vec<Vec<Vec<Side>>> = vec![vec![Vec::new();4];group_id];
    for index in 0..map.len(){ 
        let group_id = groups.get(&index).unwrap();     
        for direction in 0..4 {
            if has_normal(&index, &direction, &map, &max) {
                let mut is_new = true;
                if region_sides[*group_id][direction].len() != 0 {
                    for side in region_sides[*group_id][direction].iter() {
                        if is_part_of_side(&index, side, &max){
                            is_new = false;
                            break;
                        }
                    }
                }
                if is_new {
                    let mut side = Side{index: index, length_right:1, normal_direction: direction};
                    while extend_side_left(&mut side, &map, &max){};
                    while extend_side_right(&mut side, &map, &max){};
                    region_sides[*group_id][direction].push(side);
                }
            }
        }
    }
    let mut region_side_numbers: Vec<usize> = vec![0; group_id];
    for id in 0..group_id {
        region_side_numbers[id] = region_sides[id][0].len() + region_sides[id][1].len() + region_sides[id][2].len() + region_sides[id][3].len();
    }
    println!("sides: {:?}", region_side_numbers);

    let result: usize = region_areas.iter().zip(region_side_numbers.iter()).map(|(areas, sides)| areas * sides).sum();

    println!("Result: {:?}", result);
    return Ok(());
}