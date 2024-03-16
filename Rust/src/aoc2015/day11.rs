
pub fn run() {
    let input = "hepxcrrq";
    let input_vec: Vec<u32> = input.chars()
        .map(|c| (c as u32) - ('a' as u32))
        .collect();

    println!("Part 1: {}", part_one(input_vec.clone()));
}

fn convert_to_string(input_vec: &[u32]) -> String {
    input_vec.iter()
        .map(|&num| ((num) as u8 + 'a' as u8) as char) // Convert each number to a character
        .collect() // Collect characters into a String
}

fn contains_straight(input_vec: &[u32]) -> bool {
    input_vec.windows(3).any(|window| window[0] + 1 == window[1] && window[1] + 1 == window[2])
}

fn contains_lio(input_vec: &[u32]) -> bool {
    input_vec.iter().any(|&c| c == ('i' as u32 - 'a' as u32) || c == ('o' as u32 - 'a' as u32) || c == ('l' as u32 - 'a' as u32))
}

fn contains_pairs(input_vec: &[u32]) -> bool {
    let mut pairs = 0;
    let mut i = 0;
    while i < input_vec.len() - 1 {
        if input_vec[i] == input_vec[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    pairs >= 2
}

fn is_valid(input_vec: &[u32]) -> bool {
    contains_straight(input_vec) && !contains_lio(input_vec) && contains_pairs(input_vec)
}

fn increment(input_vec: Vec<u32>) -> Vec<u32> {
    let mut i = input_vec.len() as i32 - 1;
    let mut input_vec = input_vec;
    while i >= 0 {
        if input_vec[i as usize] == 25 {
            input_vec[i as usize] = 0;
            i -= 1;
        } else {
            input_vec[i as usize] += 1;
            break;
        }
    }
    input_vec
}

fn part_one(input_vec: Vec<u32>) -> String {

    let mut input_vec = input_vec;
    loop {
        input_vec = increment(input_vec);
        if is_valid(&input_vec) {
            return convert_to_string(&input_vec);
        }
    }

}

fn part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment(vec![0, 0, 0]), vec![0, 0, 1]);
        assert_eq!(increment(vec![0, 0, 25]), vec![0, 1, 0]);
        assert_eq!(increment(vec![0, 25, 25]), vec![1, 0, 0]);
        assert_eq!(increment(vec![25, 25, 25]), vec![0, 0, 0]);
    }

    #[test] 
    fn test_is_valid() {
        let input = "abcdffaa";
        let input_vec: Vec<u32> = input.chars()
            .map(|c| (c as u32) - ('a' as u32))
            .collect();
        assert_eq!(is_valid(&input_vec), true);
    }
}
