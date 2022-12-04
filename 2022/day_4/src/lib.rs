use itertools::Itertools;
use std::fs::read_to_string;

fn str_to_range(s: &str) -> Option<(u32, u32)> {
    s.trim()
        .split('-')
        .map(|r| r.parse::<u32>().unwrap())
        .collect_tuple()
}

fn full_overlap_check(elf1: &(u32, u32), elf2: &(u32, u32)) -> u32 {
    ((elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1)) as u32
}

fn partial_overlap_check(elf1: &(u32, u32), elf2: &(u32, u32)) -> u32 {
    (((elf1.0 >= elf2.0 && elf1.0 <= elf2.1) || (elf1.1 >= elf2.0 && elf1.1 <= elf2.1))
        || ((elf2.0 >= elf1.0 && elf2.0 <= elf1.1) || (elf2.1 >= elf1.0 && elf2.1 <= elf1.1)))
        as u32
}

pub fn fully_overlapping_pairs(path: &str) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|elves| elves.split(',').collect_tuple().unwrap())
        .filter_map(|(elf1, elf2)| {
            if let (Some(elf1), Some(elf2)) = (str_to_range(elf1), str_to_range(elf2)) {
                Some(full_overlap_check(&elf1, &elf2))
            } else {
                None
            }
        })
        .sum()
}

pub fn partial_overlapping_pairs(path: &str) -> u32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|elves| elves.split(',').collect_tuple().unwrap())
        .filter_map(|(elf1, elf2)| {
            if let (Some(elf1), Some(elf2)) = (str_to_range(elf1), str_to_range(elf2)) {
                Some(partial_overlap_check(&elf1, &elf2))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_overlapping_pairs_test() {
        let result = fully_overlapping_pairs("example.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn partial_overlapping_pairs_test() {
        let result = partial_overlapping_pairs("example.txt");
        assert_eq!(result, 4);
    }
}
