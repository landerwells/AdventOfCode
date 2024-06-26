use crate::utils::*;

pub fn run() {
    let input = get_input(2023, 6);

    println!("Part One: {}", part_one(&input, false));
    println!("Part Two: {}", part_one(&input, true));
}

fn part_one(input: &String, part_two: bool) -> i128 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut times: Vec<i128> = Vec::new();
    let mut distances: Vec<i128> = Vec::new();

    if !part_two {
        times = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        distances = lines[1]
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
    } else {
        times = vec![lines[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i128>()
            .unwrap()];
        distances = vec![lines[1]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i128>()
            .unwrap()];
    }

    let mut current_ways = 0;
    let mut total_ways = 1;

    for time in 0..times.len() {
        for i in 0..times[time] {
            let rev = times[time] - i;
            if (rev * i) > distances[time] {
                current_ways += 1;
            }
        }
        total_ways *= current_ways;
        current_ways = 0;
    }

    total_ways
}
