use crate::utils::*;

pub fn run() {
    let input: String = get_input(2023, 1);
    println!("Part One: {}", parse_input(&input, false));
    println!("Part Two: {}", parse_input(&input, true));
}

fn parse_input(input: &String, replace: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if replace {
                line.to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
    .map(|line| {
        line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
    })
    .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}
