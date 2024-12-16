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
fn calc_hash(index: usize, direction:u8)-> usize {
    return 4*index + direction as usize;
}

#[inline]
fn calc_node_from_hash(hash:usize) -> (usize, u8) {
    return ((hash-(hash%4))/4, (hash%4) as u8)
}

fn dijkstra(index_start: &usize, index_end: &usize, map: &Map) -> (Vec<Vec<usize>>, usize) {
    let mut queue = Vec::new();
    for index in 0..map.fields.len(){
        if map.fields[index] == '#' {
            continue;
        }
        for direction in 00..4_u8{
            queue.push(calc_hash(index, direction));
        }
    }
    let mut dist = vec![usize::MAX; map.fields.len()*4];
    dist[calc_hash(*index_start, 1_u8)] = 0;
    let mut prev = vec![Vec::<usize>::with_capacity(4); map.fields.len()*4];

    queue.sort_by(|&a,&b| dist[a].cmp(&dist[b]));
    
    while queue.len() != 0 {
        let hash_current= queue[0];
        let (index, direction_current) = calc_node_from_hash(hash_current);
        queue.remove(0);

        if index >= map.fields.len() || direction_current > 3 {
            println!("Error: {}, {} (from: {})", index, direction_current, hash_current);
        }

        if index == *index_end {
            break;
        }
        if dist[hash_current] == usize::MAX {
            break;
        }
        
        for direction in 0..4_u8 {
            if !can_walk_to(&index, direction, map){
                continue;
            }
            let cost : usize;
            let hash_next: usize;
            if direction == direction_current {
                cost = 1;
                hash_next = calc_hash(next_index(&index, direction, &map.max), direction)
            } else {
                cost = 1000;
                hash_next = calc_hash(index, direction)
            }
            if hash_current >= dist.len() {
                println!("current: {hash_current} vs {}", dist.len());
            }
            if hash_next >= dist.len(){
                println!("next: {hash_next} vs {}", dist.len());
            }
            if dist[hash_next] >= dist[hash_current] +cost {
                if dist[hash_next] > dist[hash_current] +cost {
                    dist[hash_next] = dist[hash_current] +cost;
                    prev[hash_next].clear();
                }
                prev[hash_next].push(hash_current);
            }  
        }
        queue.sort_by(|&a,&b| dist[a].cmp(&dist[b]));
        //direction_current = calc_direction(&index, &stack[0], &map.max);
    }
    let mut cost = usize::MAX;
    for direction in 0..4_u8{
        if cost > dist[calc_hash(*index_end, direction)] {
            cost = dist[calc_hash(*index_end, direction)];
        }
    }
    return (prev, cost)
}

fn _insert_used(hash: usize, prevs:&Vec<Vec<usize>>, map_with_path: &mut Vec<char>){
    map_with_path[calc_node_from_hash(hash).0] = 'O';
    for hash_next in prevs[hash].iter(){
        _insert_used(*hash_next, prevs, map_with_path);
    }
}

fn _print_paths(prevs:&Vec<Vec<usize>>, index_end: usize, map: &Map) -> Vec<char> {
    let mut map_with_path = map.fields.clone();
    map_with_path[index_end] = 'O';
    for direction in 0..4_u8{
        for hash in prevs[calc_hash(index_end, direction)].iter(){
            _insert_used(*hash, prevs, &mut map_with_path);
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
    return map_with_path;
}

fn main() -> std::io::Result<()> {
    println!("Day 16: Task2");
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

    let(prevs, cost) = dijkstra(&index_start, &index_end,&map);
    let marked_fields = _print_paths(&prevs, index_end, &map);
    if !is_debug {
        assert_eq!(cost,109516);
    }
    println!();
    println!("{}", cost);
    println!("{}", marked_fields.iter().filter(|&&ch| ch == 'O').count());
    Ok(())
}