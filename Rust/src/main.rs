use std::env;

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2022;
mod aoc2023;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_year(&args[1]),
        3 => run_day(&args[1], &args[2]),
        _ => eprintln!("Usage: program [YEAR] or program [YEAR] [DAY]"),
    }
}

fn run_year(year: &str) {
    match year {
        "2015" => aoc2015::run_all(),
        "2016" => aoc2016::run_all(),
        "2017" => aoc2017::run_all(),
        "2022" => aoc2022::run_all(),
        "2023" => aoc2023::run_all(),
        // ... handle other years
        _ => eprintln!("Invalid year"),
    }
}

fn run_day(year: &str, day: &str) {
    match (year, day) {
        ("2015", "01") => aoc2015::day01::run(),
        ("2015", "02") => aoc2015::day02::run(),
        ("2015", "03") => aoc2015::day03::run(),
        ("2015", "05") => aoc2015::day05::run(),
        ("2015", "06") => aoc2015::day06::run(),
        ("2015", "07") => aoc2015::day07::run(),
        ("2015", "09") => aoc2015::day09::run(),
        ("2015", "10") => aoc2015::day10::run(),
        ("2015", "11") => aoc2015::day11::run(),
        ("2016", "01") => aoc2016::day01::run(),
        ("2017", "01") => aoc2017::day01::run(),
        ("2017", "02") => aoc2017::day02::run(),
        ("2017", "03") => aoc2017::day03::run(),
        ("2017", "04") => aoc2017::day04::run(),
        ("2017", "05") => aoc2017::day05::run(),
        ("2017", "06") => aoc2017::day06::run(),
        ("2022", "01") => aoc2022::day01::run(),
        ("2022", "10") => aoc2022::day10::run(),
        ("2023", "01") => aoc2023::day01::run(),
        ("2023", "02") => aoc2023::day02::run(),
        ("2023", "03") => aoc2023::day03::run(),
        ("2023", "04") => aoc2023::day04::run(),
        ("2023", "05") => aoc2023::day05::run(),
        ("2023", "06") => aoc2023::day06::run(),
        ("2023", "07") => aoc2023::day07::run(),
        ("2023", "08") => aoc2023::day08::run(),
        ("2023", "09") => aoc2023::day09::run(),
        ("2023", "10") => aoc2023::day10::run(),
        ("2023", "11") => aoc2023::day11::run(),
        ("2023", "12") => aoc2023::day12::run(),
        ("2023", "13") => aoc2023::day13::run(),
        // ... handle other years and days
        _ => eprintln!("Invalid year or day"),
    }
}
