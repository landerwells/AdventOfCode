use crate::utils::*;

pub fn run() {
    let input = get_input(2022, 1);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    *calories(input.to_string()).iter().max().unwrap()
}

fn part_two(input: &str) -> i32 {
    let mut sums: Vec<i32> = calories(input.to_string());

    sums.sort_unstable_by(|a, b| b.cmp(a));

    sums.iter().take(3).sum()
}

fn calories(input: String) -> Vec<i32> {
    let elves: Vec<String> = input.split("\n\n").map(String::from).collect();

    elves
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|line| line.to_string().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}
