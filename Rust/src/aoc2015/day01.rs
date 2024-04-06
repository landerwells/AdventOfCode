use crate::utils::*;

pub fn run() {
    let input = get_input(2015, 1);
    println!(
        "Part One: {}",
        input.chars().fold(0, |acc, c| {
            if c == '(' {
                acc + 1
            } else {
                acc - 1
            }
        })
    );
    println!("Part Two: {}", solve_part_two(&input));
}

fn solve_part_two(input: &String) -> i32 {
    let mut count = 0;
    let mut index = 1;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count == -1 {
            return index;
        }
        index += 1;
    }
    index
}
