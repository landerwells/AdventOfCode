use crate::utils::*;
use indoc::indoc;
use std::fs;

pub fn run() {
    let test_input = indoc!("");

    let input: String = fs::read_to_string("../input/2016/day1.txt").unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let instructions: Vec<String> = input.split(", ").map(|s| s.to_string()).collect();

    let mut direction = 0;
    let mut start: Coordinate = (0, 0);

    for i in instructions {}

    0
}

fn part_two(input: &String) -> i32 {
    0
}
