/*
    --- Day 1: The Tyranny of the Rocket Equation ---
    https://adventofcode.com/2019/day/1
*/

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = read_lines("input");
    let mut mass_vec: Vec<i32> = Vec::new();

    for line in lines {
        let curr_mass = line.parse::<i32>().unwrap();
        mass_vec.push(curr_mass);
    }

    let mut fuel_req = 0;
    for mass in mass_vec {
        let curr_fuel = ((mass as f32 / 3.0).floor() as i32) - 2;
        println!("{mass} -> {curr_fuel}");
        fuel_req += curr_fuel;
    }
    println!("Totel fuel requirements: {fuel_req}");
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
