use std::{fs::File, io::{BufRead, BufReader}, vec};

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Vector2D {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Vector2D,
    direction: Vector2D
}


fn calc_position_after_turns(robot: &Robot, turns: i64, max: &Vector2D) -> Vector2D {
    return Vector2D { 
        x: (max.x + ((robot.position.x + robot.direction.x * turns) % max.x))% max.x, 
        y: (max.y + ((robot.position.y + robot.direction.y * turns) % max.y))% max.y
    }
}

fn _plot_positions(robots: &Vec<Robot>, turns: i64, max: &Vector2D) {
    let mut map = vec![vec![0_usize; max.x as usize]; max.y as usize];
    for robot in robots {
        let pos = calc_position_after_turns(robot, turns, max);
        map[pos.y as usize][pos.x as usize] += 1;
    }
    for row in map{
        for field in row{
            match field {
                0 => print!("."),
                _ => print!("{}", field)
            }
        }
        println!();
    }
}

fn calc_entropy_after_turns(robots: &Vec<Robot>, turns: i64, max: &Vector2D) -> usize {
    let mut num_robots_in_quandrant = vec![0_usize;4];
    let xm = (max.x-1)/2;
    let ym = (max.y-1)/2;
    for robot in robots{
        let position = calc_position_after_turns(&robot, turns, &max);
        if position.y == ym || position.x == xm {
            continue;
        }
        if position.y < ym {
            if position.x < xm {
                num_robots_in_quandrant[0] += 1;
            } else {
                num_robots_in_quandrant[1] += 1;
            }
        } else {
            if position.x < xm {
                num_robots_in_quandrant[2] += 1;
            } else {
                num_robots_in_quandrant[3] += 1;
            }
        }
    }

    let mut result = 1_usize;
    for robots in num_robots_in_quandrant{
        result *= robots;
    }
    return result
}

fn main()  -> std::io::Result<()> {
    println!("Day 14: Task2");
    let is_debug = false;
    let file : File;
    let max: Vector2D;
    if is_debug {
        file = File::open("bin/day14/test.txt")?;
        max = Vector2D{x: 11, y: 7};
    }
    else{
        file = File::open("bin/day14/input.txt")?;
        max = Vector2D{x: 101, y: 103};
    }

    let reader = BufReader::new(file);
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = Vec::new();
    for line in reader.lines() { 
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        let cap = re.captures(&line).unwrap();
        robots.push(Robot{
            position: Vector2D { 
                x: cap.get(1).unwrap().as_str().parse().unwrap(), 
                y: cap.get(2).unwrap().as_str().parse().unwrap() 
            },
            direction: Vector2D { 
                x: cap.get(3).unwrap().as_str().parse().unwrap(), 
                y: cap.get(4).unwrap().as_str().parse().unwrap() 
            }
        });
    }
    
    let mut time_easter_egg = 0_usize;
    let mut entropy_min  = calc_entropy_after_turns(&robots, time_easter_egg as i64, &max);
    for turn in 1..10000 {
        let entropy = calc_entropy_after_turns(&robots, turn as i64, &max);
        if entropy < entropy_min {
            time_easter_egg = turn;
            entropy_min = entropy
       } 
    }

    if !is_debug {
        assert_ne!(time_easter_egg, 0);
    }
    
    _plot_positions(&robots, time_easter_egg as i64, &max);
    println!("{}", time_easter_egg);
    Ok(())
}