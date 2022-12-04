use std::collections::HashSet;

pub fn misplaced_items(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).unwrap();
    let priority_sum: u32 = input
        .lines()
        .into_iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(sack1, sack2)| {
            let items: HashSet<char> = sack1.chars().collect();
            let misplaced = sack2.chars().find(|c| items.contains(c)).unwrap();
            if misplaced.is_ascii_uppercase() {
                misplaced as u32 - 'A' as u32 + 27
            } else {
                misplaced as u32 - 'a' as u32 + 1
            }
        })
        .sum();
    priority_sum
}

pub fn misplaced_badges(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).unwrap();
    let badge_sum: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .into_iter()
        .map(|elves| {
            let elf1 = elves[0];
            let elf2 = elves[1];
            let elf3 = elves[2];
            let badge = elf1
                .chars()
                .find(|c| elf2.contains(*c) && elf3.contains(*c))
                .unwrap();
            if badge.is_ascii_uppercase() {
                badge as u32 - 'A' as u32 + 27
            } else {
                badge as u32 - 'a' as u32 + 1
            }
        })
        .sum();
    badge_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misplaced_items_test() {
        let result = misplaced_items("example.txt");
        assert_eq!(result, 157);
    }

    #[test]
    fn misplaced_badges_test() {
        let result = misplaced_badges("example.txt");
        assert_eq!(result, 70);
    }
}
