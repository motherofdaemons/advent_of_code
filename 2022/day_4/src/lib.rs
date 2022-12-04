use std::ops::Index;

pub fn fully_overlapping_pairs(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).unwrap();
    let total: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let elves = line
                .split(',')
                .map(|elf| {
                    *elf.split('-')
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .windows(2)
                        .map(|a| (a[0], a[1]))
                        .collect::<Vec<(u32, u32)>>()
                        .first()
                        .unwrap()
                })
                .collect::<Vec<(u32, u32)>>();
            let elf1 = elves.index(0);
            let elf2 = elves.index(1);
            ((elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1))
                as u32
        })
        .sum();
    total
}

pub fn partial_overlapping_pairs(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).unwrap();
    let total: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let elves = line
                .split(',')
                .map(|elf| {
                    *elf.split('-')
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .windows(2)
                        .map(|a| (a[0], a[1]))
                        .collect::<Vec<(u32, u32)>>()
                        .first()
                        .unwrap()
                })
                .collect::<Vec<(u32, u32)>>();
            let elf1 = elves.index(0);
            let elf2 = elves.index(1);
            (((elf1.0 >= elf2.0 && elf1.0 <= elf2.1) || (elf1.1 >= elf2.0 && elf1.1 <= elf2.1))
                || ((elf2.0 >= elf1.0 && elf2.0 <= elf1.1)
                    || (elf2.1 >= elf1.0 && elf2.1 <= elf1.1))) as u32
        })
        .sum();
    total
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
