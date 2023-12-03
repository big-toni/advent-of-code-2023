use std::fs;
use std::collections::HashMap;

pub fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Failed to read input file");

    let result = solve(&input);
    println!("Sum of IDs of possible games: {}", result);
}

fn solve(input: &str) -> i32 {
    let mut sum_of_ids = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let game_data = parts[1];

        if is_game_possible(game_data) {
            sum_of_ids += game_id;
        }
    }

    sum_of_ids
}
// 2727

fn is_game_possible(game_data: &str) -> bool {
    let sets: Vec<&str> = game_data.split("; ").collect();
    let mut max_cubes = HashMap::new();

    max_cubes.insert("red", 12);
    max_cubes.insert("green", 13);
    max_cubes.insert("blue", 14);

    for set in sets {
        let cubes: Vec<&str> = set.split(", ").collect();
        for cube in cubes {
            let parts: Vec<&str> = cube.split_whitespace().collect();
            let count: i32 = parts[0].parse().unwrap();
            let color = parts[1];

            if let Some(&max_count) = max_cubes.get(color) {
                if count > max_count {
                    return false;
                }
            }
        }
    }

    true
}