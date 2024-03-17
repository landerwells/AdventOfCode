use std::fs;
pub type Coordinate = (i32, i32);

pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

pub fn get_distance(from: Coordinate, to: Coordinate) -> i32 {
    let distance: i32 = (from.0 - to.0).abs() + (from.1 - to.1).abs();
    distance
}

pub fn get_input(year: i32, day: i32) -> String {
    let year: String = year.to_string();
    let day: String = day.to_string();
    let source: String = format!("../inputs/{}/{}", year, day);
    let input: String = fs::read_to_string(source).unwrap();
    input
}
