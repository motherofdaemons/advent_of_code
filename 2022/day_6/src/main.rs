use std::fs::read_to_string;

use day_6::find_first_marker;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", find_first_marker(input.as_str(), 4));
    println!("Part 2: {}", find_first_marker(input.as_str(), 14));
}