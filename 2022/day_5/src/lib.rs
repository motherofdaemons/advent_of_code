use itertools::Itertools;
use std::fs::read_to_string;

pub fn crate_mover_9000(path: &str) -> String {
    let input = read_to_string(path).unwrap();
    let (crates_str, moves) = input.split("\n\n").collect_tuple().unwrap();
    let crate_stacks_len = crates_str
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut crate_stack: Vec<Vec<char>> = Vec::new();
    crate_stack.resize(crate_stacks_len, Vec::new());
    for line in crates_str.lines().rev() {
        for (index, value) in line
            .chars()
            .enumerate()
            .filter(|(i, _)| *i % 4 == 1)
            .map(|(_, e)| e)
            .enumerate()
        {
            if value.is_alphabetic() {
                crate_stack[index].push(value);
            }
        }
    }

    for m in moves.lines() {
        let words: Vec<&str> = m.split(' ').collect();
        let num_to_move = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..num_to_move {
            let c = crate_stack[from].pop().unwrap();
            crate_stack[to].push(c);
        }
    }

    crate_stack
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

pub fn crate_mover_9001(path: &str) -> String {
    let input = read_to_string(path).unwrap();
    let (crates_str, moves) = input.split("\n\n").collect_tuple().unwrap();
    let crate_stacks_len = crates_str
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut crate_stack: Vec<Vec<char>> = Vec::new();
    crate_stack.resize(crate_stacks_len, Vec::new());
    for line in crates_str.lines().rev() {
        for (index, value) in line
            .chars()
            .enumerate()
            .filter(|(i, _)| *i % 4 == 1)
            .map(|(_, e)| e)
            .enumerate()
        {
            if value.is_alphabetic() {
                crate_stack[index].push(value);
            }
        }
    }

    for m in moves.lines() {
        let words: Vec<&str> = m.split(' ').collect();
        let num_to_move = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        let drain_range = crate_stack[from].len()-num_to_move ..;
        let mut stack: Vec<char> = crate_stack[from].drain(drain_range).collect();
        crate_stack[to].append(&mut stack);
    }

    crate_stack
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_mover_9000_test() {
        let result = crate_mover_9000("example.txt");
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn crate_mover_9001_test() {
        let result = crate_mover_9001("example.txt");
        assert_eq!(result, "MCD");
    }
}
