use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize
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

fn flush(index: &usize, map: &Vec<char>, max: &Coordinate, group_id: &mut usize, groups: &mut HashMap<usize, usize>) -> Region {
    groups.insert(*index, *group_id);
    //println!("{}|{}",calc_index_x(index, max), calc_index_y(index, max));
    let mut region = Region{area: 1, perimeter: 0};
    for direction in 0..4{
        let next_index = next_index(&index, &direction, &max);
        if next_index == *index {
            region.perimeter += 1;
            continue;
        }
        if map[*index] != map[next_index]{
            region.perimeter += 1;
            continue;
        }
        let id =  groups.get(&next_index);
        if id.is_none(){
            let new_region = flush(&next_index, map, max, group_id, groups);
            region.area += new_region.area;
            region.perimeter += new_region.perimeter;
        } 
    }
    return region;
}


fn main() -> std::io::Result<()> {
    println!("Day 12: Task1");
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
    let mut regions: HashMap<usize, Region> = HashMap::new();

    let mut group_id = 0;
    for index in 0..map.len(){
        if groups.get(&index).is_none(){
            group_id += 1;
            regions.insert(group_id, flush(&index, &map, &max, &mut group_id, &mut groups));
        }
    }
    // println!("regions: {:?}", regions);

    let result: usize = regions.iter().map(|(_, region)| region.area * region.perimeter).sum();

    println!("Result: {:?}", result);
    return Ok(());
}