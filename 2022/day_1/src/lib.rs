use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

fn most_calories(path: &str) -> i32 {
    let mut current_calories = 0;
    let mut max_calories = 0;
    let lines = read_lines(path).unwrap();
    for line in lines.flatten() {
        if let Ok(calories) = line.parse::<i32>() {
            current_calories += calories;
        } else {
            max_calories = max(current_calories, max_calories);
            current_calories = 0;
        }
    }
    max(current_calories, max_calories)
}

fn top_three_calories(path: &str) -> i32 {
    let mut current_calories = 0;
    let mut calories_by_elf = Vec::new();
    let lines = read_lines(path).unwrap();
    for line in lines.flatten() {
        if let Ok(calories) = line.parse::<i32>() {
            current_calories += calories;
        } else {
            calories_by_elf.push(current_calories);
            current_calories = 0;
        }
    }
    calories_by_elf.push(current_calories);
    calories_by_elf.sort();
    calories_by_elf.iter().rev().take(3).sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_most_calories() {
        let result = most_calories("food.txt");
        assert_eq!(result, 24000);
    }

    #[test]
    fn example_top_three_calories() {
        let result = top_three_calories("food.txt");
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_most_calories() {
        let result = most_calories("input.txt");
        assert_eq!(result, 70720);
    }

    #[test]
    fn test_top_three_calories() {
        let result = top_three_calories("input.txt");
        assert_eq!(result, 207148);
    }
}
