use crate::utils::*;

pub fn run() {
    let input: String = get_input(2017, 2);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let mut nums: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        nums.sort();
        sum += nums[nums.len() - 1]- nums[0];
    }

    sum

}

fn part_two(input: &String) -> i32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] % nums[j] == 0 && i != j {
                    sum += nums[i] / nums[j];
                }
            }
        }
    }

    sum
}
