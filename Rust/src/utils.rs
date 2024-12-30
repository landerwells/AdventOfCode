use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};
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

pub fn get_input(year: i32, day: i32) -> String {
    let year = year.to_string();
    let day = day.to_string();
    let source = format!("~/Developer/AdventOfCode/inputs/{}/{}", year, day);
    let source_path = expand_path(&source);
    let input = fs::read_to_string(source_path).unwrap();
    input
}

fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = home_dir() {
            return home.join(&path[2..]);
        }
    }
    Path::new(path).to_path_buf()
}

pub fn get_distance(from: Coordinate, to: Coordinate) -> i32 {
    let distance: i32 = (from.0 - to.0).abs() + (from.1 - to.1).abs();
    distance
}
