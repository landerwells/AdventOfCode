use crate::utils::*;
use std::collections::HashSet;

// Could definitely improve the speed on this day
pub fn run() {
    let input = get_input(2023, 4);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut answer = 0;
    for line in &lines {
        answer += line_points(line.clone());
    }
    answer
}

// This method is so god damn slow
fn part_two(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut line_number = 0;
    let mut total_count = 0;
    let mut winning_numbers_vector: Vec<i32> = Vec::new();

    for line in &lines {
        winning_numbers_vector.push(winning_numbers(line.clone()));
    }

    // Vector to count how many cards total we have
    let mut cards: Vec<i32> = vec![1; lines.len()];
    for _line in &lines {
        while cards[line_number] != 0 {
            let current_points: usize = winning_numbers_vector[line_number] as usize;
            for i in 0..current_points {
                cards[line_number + i + 1] += 1;
            }
            cards[line_number] -= 1;
            total_count += 1;
        }
        line_number += 1;
    }
    total_count
}

fn line_points(line: String) -> i32 {
    let mut current_points = 0;

    for _i in 0..winning_numbers(line) {
        if current_points == 0 {
            current_points += 1;
        } else {
            current_points *= 2;
        }
    }
    current_points
}

fn winning_numbers(line: String) -> i32 {
    let mut set = HashSet::new();
    let mut current_points = 0;
    let mut first_half = true;
    let parts: Vec<&str> = line.split_whitespace().collect();

    for part in parts {
        if part == "|" {
            first_half = false;
        }
        if first_half {
            set.insert(part);
        } else if set.contains(part) {
            current_points += 1;
        }
    }
    current_points
}
