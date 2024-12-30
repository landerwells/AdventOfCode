use crate::utils::*;
use std::collections::HashSet;

pub fn run() {
    let input: String = get_input(2017, 6);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut input: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let mut steps = 0;

    loop {
        if !set.insert(input.clone()) {
            break;
        }

        // Find the index of the max element
        let (max_index, max_value) = input.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();

        // Redistribute for max_value amount
        input[max_index] = 0;
        distribute(&mut input, max_index, *max_value);

        steps += 1;
    }

    steps
}

fn distribute(input: &mut Vec<i32>, start: usize, amount: i32) -> Vec<i32> {
    let len = input.len();
    let mut current_position = start;
    let mut remaining_amount = amount;

    while remaining_amount > 0 {
        input[current_position % len] += 1;
        current_position += 1;
        remaining_amount -= 1;
    }

    input
}

fn part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_distribute() {
        assert_eq!(vec![2, 4, 1, 2], distribute(vec![0, 2, 0, 0], 3, 7));
    }
}
