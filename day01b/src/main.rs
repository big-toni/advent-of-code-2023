use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|c| c.is_digit(10));

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let num_str = format!("{}{}", first, last);
            println!("{:?} -> {:?}", line, num_str );
            if let Ok(num) = num_str.parse::<i32>() {
                sum += num;
            }
        }
    }
    println!("Sum of calibration values: {}", sum);
}