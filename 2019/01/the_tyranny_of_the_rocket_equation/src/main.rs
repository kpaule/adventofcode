/*
    --- Day 1: The Tyranny of the Rocket Equation ---
    https://adventofcode.com/2019/day/1
*/

mod rocket;

use crate::rocket::{Module, Rocket};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let module_weights = read_lines("input");

    let mut rocket = Rocket::new();

    for weight in module_weights {
        rocket.load(Module::new(weight));
    }

    println!("Totel fuel requirements: {}", rocket.fuel_requirement());

    println!(
        "Total fuel for fuel requirements: {}",
        rocket.fuel_for_fuel_requirement()
    );
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect()
}
