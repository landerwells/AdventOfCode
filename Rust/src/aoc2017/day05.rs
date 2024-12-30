use crate::utils::*;

pub fn run() {
    let input: String = get_input(2017, 5);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut nums: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut counter = 0;
    let mut offset: i32 = 0;

    loop {
        if offset as usize >= nums.len() || offset < 0 {
            return counter;
        }

        nums[offset as usize] += 1;
        offset += nums[offset as usize] - 1;

        counter += 1;
    }
}

fn part_two(input: &String) -> i32 {
    let mut nums: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut counter = 0;
    let mut offset: i32 = 0;

    loop {
        if offset as usize >= nums.len() || offset < 0 {
            return counter;
        } else if nums[offset as usize] >= 3 {
            nums[offset as usize] -= 1;
            offset += nums[offset as usize] + 1;
        } else {
            nums[offset as usize] += 1;
            offset += nums[offset as usize] - 1;
        }

        counter += 1;
    }
}
