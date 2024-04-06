use crate::utils::*;

pub fn run() {

    let input = get_input(2015, 5);
    println!("Part One: {}", part_one(&input));
    // Part two was infinitely easier to implement using grep and regex,
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut count = 0;
    for line in &lines {
        if is_nice(line) {
            count += 1;
        }
    }
    count
}

fn is_nice(line: &String) -> bool {
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }

    if line
        .chars()
        .collect::<Vec<_>>() // Collect into a vector to use `windows`
        .windows(2)
        .any(|window| window[0] == window[1])
        && line
            .chars()
            .filter(|&c| "aeiou".contains(c)) // Simplified vowel check
            .count()
            >= 3
    {
        return true;
    }
    false
}
