use crate::utils::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input: String = get_input(2017, 4);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut set = HashSet::new();
    let input = input
        .lines() // Split the string into lines
        .map(|line| {
            line.split_whitespace() // Split each line into words
                .map(|word| word.to_string()) // Convert each word to String
                .collect::<Vec<String>>()
        }) // Collect words into a Vec<String>
        .collect::<Vec<Vec<String>>>(); // Collect lines into a Vec<Vec<String>>

    let mut count = 0;
    for line in input {
        let mut contains = false;
        for word in line {
            if set.contains(&word) {
                contains = true;
                break;
            }
            set.insert(word);
        }
        if !contains {
            count += 1;
        }
        contains = false;
        set.clear();
    }

    count
}

fn part_two(input: &String) -> i32 {
    let input = input
        .lines() // Split the string into lines
        .map(|line| {
            line.split_whitespace() // Split each line into words
                .map(|word| word.to_string()) // Convert each word to String
                .collect::<Vec<String>>()
        }) // Collect words into a Vec<String>
        .collect::<Vec<Vec<String>>>(); // Collect lines into a Vec<Vec<String>>

    let mut is_valid = true;
    let mut sum = 0;

    for line in input {
        for x in 0..line.len() {
            for y in 0..line.len() {
                if is_anagram(line[x].clone(), line[y].clone()) && x != y {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            sum += 1;
        }
        is_valid = true;
    }
    sum
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_counts = HashMap::new();
    let mut t_counts = HashMap::new();

    s.chars().for_each(|c| *s_counts.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *t_counts.entry(c).or_insert(0) += 1);

    s_counts == t_counts
}
