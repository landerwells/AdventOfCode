use crate::utils::*;

pub fn run() {
    let input = get_input(2016, 1);

    println!("Part One: {}", part_one(&input));
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
