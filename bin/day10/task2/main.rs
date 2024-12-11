use std::{fs::File, io::{BufRead, BufReader}};

struct Coordinate {
    x: usize,
    y: usize
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

fn calc_rating(map: &Vec<u8>, index: &usize, max: &Coordinate) -> usize {
    //println!("{index} ({}): ({}|{})", map[*index], calc_index_x(index, max), calc_index_y(index, max));
    if map[*index] == 9 {
        //println!("    success!");
        return 1;
    }
    let mut rating = 0;
    for direction in 0 ..4 {
        //print!("    dir: {direction}: ");
        let next_index = next_index(index, &direction, max);
        if next_index == *index {
            //println!("not exists!");
            continue;
        }
        if map[next_index] != map[*index] +1 {
            //println!("no increase!");
            continue;
        }
        //println!("continue with: {next_index}");
        rating += calc_rating(map, &next_index, max);
    }
    return rating;
}

fn main() -> std::io::Result<()> {
    println!("Day 10: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day10/test.txt")?;
    }
    else{
        file = File::open("bin/day10/input.txt")?;
    }
    let reader = BufReader::new(file);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = Vec::new();
    let mut zeros = Vec::new();
    
    let mut index = 0usize;
    for line in reader.lines() {
        let line = line?;
        max_x = line.len();
        for field in line.chars(){
            let num = field.to_digit(10).unwrap() as u8;
            if num == 0 {
                zeros.push(index);
            }
            map.push(num);
            index += 1;
        }
        max_y += 1;
    }
    let max = Coordinate{x: max_x, y: max_y};
    // println!("zeros: {:?}", zeros);
    //println!("{}", max.y);
    //println!("result: {}", get_trailheads(&map, &zeros[0], &max));


    let trailhead_ratings: usize = zeros.iter().map(|index| {
        calc_rating(&map, index, &max)
    }
    ).sum();

    println!("ratings: {:?}", trailhead_ratings);
    return Ok(());
}