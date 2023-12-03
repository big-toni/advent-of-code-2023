use std::fs;
use std::collections::HashMap;

pub fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Failed to read input file");

    let result = solve(&input);
    println!("Sum of the powers of the minimum sets: {}", result);
}

fn solve(input: &str) -> i64 {
    input.lines().map(|line| {
        let game_data = line.split(": ").nth(1).unwrap();
        let min_cubes = find_min_cubes(game_data);
        min_cubes.values().map(|&v| v as i64).product::<i64>()
    }).sum()
}

fn find_min_cubes(game_data: &str) -> HashMap<&str, i32> {
    let mut min_cubes = HashMap::new();
    min_cubes.insert("red", 0);
    min_cubes.insert("green", 0);
    min_cubes.insert("blue", 0);

    for set in game_data.split("; ") {
        for cube in set.split(", ") {
            let parts: Vec<&str> = cube.split_whitespace().collect();
            let count: i32 = parts[0].parse().unwrap();
            let color = parts[1];

            let entry = min_cubes.entry(color).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    min_cubes
}
// 56580