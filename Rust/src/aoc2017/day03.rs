use crate::utils::*;
use std::collections::HashMap;

pub fn run() {
    let input: i32 = 277678;

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

fn part_one(input: i32) -> i32 {
    let mut map: HashMap<i32, Coordinate> = HashMap::new();
    let mut max_steps = 0;
    let mut turns = 0;
    let mut current_coordinate = (0, 0);
    let mut direction = 0;
    let mut steps = 0;
    // map.insert(1, (0, 0));

    for i in 1..=input {
        map.insert(i, current_coordinate);
        if steps == max_steps {
            turns += 1;
            steps = 0;
            if turns % 2 == 1 {
                max_steps += 1;
            }
            direction = (direction + 1) % 4
        }
        steps += 1;
        match direction {
            0 => current_coordinate = (current_coordinate.0 + 1, current_coordinate.1),
            1 => current_coordinate = (current_coordinate.0, current_coordinate.1 + 1),
            2 => current_coordinate = (current_coordinate.0 - 1, current_coordinate.1),
            3 => current_coordinate = (current_coordinate.0, current_coordinate.1 - 1),
            _ => panic!("Bruh"),
        }
    }

    get_distance((0, 0), *map.get(&input).unwrap())
}

fn part_two(input: i32) -> i32 {
    0
}
