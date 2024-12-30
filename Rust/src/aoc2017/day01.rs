use crate::utils::*;

pub fn run() {
    let input: String = get_input(2017, 1);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut vec: Vec<i32> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    vec.push(vec[0]);
    let mut sum = 0;

    for i in 0..(vec.len() - 1) {
        if vec[i] == vec[i + 1] {
            sum += vec[i];
        }
    }

    sum
}

fn part_two(input: &String) -> i32 {
    let vec: Vec<i32> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    let length = vec.len();
    let mut sum = 0;

    for i in 0..(vec.len()) {
        if vec[i] == vec[(i + length / 2) % length] {
            sum += vec[i];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&String::from("1212")), 6);
        assert_eq!(part_two(&String::from("1221")), 0);
        assert_eq!(part_two(&String::from("123425")), 4);
        assert_eq!(part_two(&String::from("123123")), 12);
        assert_eq!(part_two(&String::from("12131415")), 4);
    }
}
