use nalgebra::{Matrix2, Vector2};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::usize;

#[derive(Clone, Copy, Debug)]
struct Vector2D {
    x: usize,
    y: usize,
}

fn vec2d_from_string(regex: & Regex, string: &String) -> Vector2D {
    let caps = regex.captures(&string).unwrap();
    let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    Vector2D { x: x, y: y}
}

#[derive(Clone, Copy, Debug)]
struct Arcade {
    goal: Vector2D,
    button_a: Vector2D,
    button_b: Vector2D
}

fn calc_min_spend(arcade: &Arcade) -> usize {
    //println!("{:?}", arcade);
    let a = Matrix2::new(
        arcade.button_a.x as f64, arcade.button_b.x as f64,  
        arcade.button_a.y as f64, arcade.button_b.y as f64,
    );
    let b = Vector2::new(arcade.goal.x as f64, arcade.goal.y as f64);
    let decomb = a.lu();
    let x = decomb.solve(&b).expect("Linear resolution failed.");
    let result : Vec<&f64> = x.iter().collect();
    //println!("{:?}", result);
    if (result[0].round() * arcade.button_a.x as f64 + result[1].round() * arcade.button_b.x as f64) as usize != arcade.goal.x {
        return 0;
    }
    if (result[0].round() * arcade.button_a.y as f64 + result[1].round() * arcade.button_b.y as f64) as usize !=  arcade.goal.y {
        return 0;
    }
    let cost = (result[0].round()  * 3.0 + result[1].round()) as usize;
    //println!("Got price for: {} coins", cost);
    return cost;
}

fn main()  -> std::io::Result<()> {
    println!("Day 13: Task2");
    let is_debug = false;
    let file : File;
    if is_debug {
        file = File::open("bin/day13/test.txt")?;
    }
    else{
        file = File::open("bin/day13/input.txt")?;
    }

    let reader = BufReader::new(file);
    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let re_goal = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let mut arcades = Vec::new();
    let mut arcade_input_counter = 0_u8;
    let mut arcade = Arcade{goal: Vector2D { x: 0, y: 0 }, button_a: Vector2D { x: 0, y: 0 }, button_b: Vector2D { x: 0, y: 0 }};
    for line in reader.lines() { 
        let line = line?;
        if line.len() == 0 {
            arcade_input_counter = 0;
            continue;
        }
        match arcade_input_counter {
            0 => arcade.button_a = vec2d_from_string(&re_button, &line),
            1 => arcade.button_b = vec2d_from_string(&re_button, &line),
            2 => {
                arcade.goal = vec2d_from_string(&re_goal, &line);
                arcade.goal.x += 10000000000000;
                arcade.goal.y += 10000000000000;
                arcades.push(arcade.clone());
            },
            _ => println!("Something went wrong!"),
        }
        
        arcade_input_counter += 1;
    }
    let mut money = vec![0; arcades.len()];
    for (index,arcade) in arcades.iter().enumerate(){
        money[index] = calc_min_spend(arcade);
    }

    println!("{}", money.iter().sum::<usize>());
    Ok(())
}