use crate::utils::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input: String = get_input(2015, 9);

    let mut distances = HashMap::new();
    let mut cities = HashSet::new();

    // Parse the data
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let distance = parts[1].parse::<u32>().unwrap();
        let cities_pair: Vec<&str> = parts[0].split(" to ").collect();

        cities.insert(cities_pair[0].to_string());
        cities.insert(cities_pair[1].to_string());

        distances.insert((cities_pair[0].to_string(), cities_pair[1].to_string()), distance);
        distances.insert((cities_pair[1].to_string(), cities_pair[0].to_string()), distance);
    }

    let cities: Vec<String> = cities.into_iter().collect();
    let mut visited = HashSet::new();
    let mut min_distance = u32::MAX;
    // let mut min_distance = 0;

    for city in &cities {
        visited.insert(city.to_string());
        dfs(&city, &cities, &mut visited, 0, &distances, &mut min_distance);
        visited.remove(city);
    }

    println!("Shortest path distance: {}", min_distance);
}

fn dfs(city: &str, cities: &[String], visited: &mut HashSet<String>, current_distance: u32, distances: &HashMap<(String, String), u32>, min_distance: &mut u32) {
    if visited.len() == cities.len() {
        // *min_distance = (*min_distance).max(current_distance);
        *min_distance = (*min_distance).min(current_distance);
        return;
    }

    for next_city in cities {
        if !visited.contains(next_city) {
            visited.insert(next_city.to_string());
            let distance_to_next = distances.get(&(city.to_string(), next_city.to_string())).unwrap_or(&0);
            dfs(next_city, cities, visited, current_distance + distance_to_next, distances, min_distance);
            visited.remove(next_city);
        }
    }
}
