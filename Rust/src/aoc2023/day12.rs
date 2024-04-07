use crate::utils::*;

pub fn run() {
    let input = get_input(2023, 12);

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(_input: &String) -> i32 {
    0
}

fn part_two(_input: &String) -> i32 {
    0
}

fn is_valid(groups: Vec<&str>, counts: Vec<i32>) -> bool {
    groups.len() == counts.len()
        && groups
            .iter()
            .zip(counts.iter())
            .all(|(g, &c)| g.len() as i32 == c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(vec!["#", "##", "###"], vec![1, 2, 3]));
        assert!(is_valid(vec!["#", "###", "#"], vec![1, 3, 1]));
    }
}
