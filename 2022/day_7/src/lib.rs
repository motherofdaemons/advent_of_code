use std::{collections::HashMap, fs::read_to_string};

fn generate_dirs(input: &str) -> HashMap<String, usize> {
    let mut dirs = HashMap::new();
    let mut path = Vec::new();

    for line in input.lines() {
        // we don't have to do anything with ls or dir becasuse we will handle it later
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        match line.split_whitespace().collect::<Vec<_>>()[..] {
            // moving up the structure
            ["$", "cd", ".."] => {
                path.pop();
            }
            // moving down the structure
            ["$", "cd", name] => {
                path.push(name);
            }
            // We don't care about individual files we only care about total space
            [size, _] => {
                // Get the size
                let size: usize = size.parse().unwrap();
                // Update everyone that is currently in scope
                for idx in 0..path.len() {
                    let path = "/".to_string() + path[1..=idx].join("/").as_str();
                    *dirs.entry(path).or_insert(0) += size;
                }
            }
            _ => unreachable!(),
        }
    }
    dirs
}

pub fn find_small_dirs(path: &str) -> usize {
    const MAX_SIZE: usize = 100_000;

    let input = read_to_string(path).unwrap();
    let dirs = generate_dirs(input.as_str());

    // get all of them below the cut off and total it up
    dirs.into_values().filter(|size| *size <= MAX_SIZE).sum()
}

pub fn find_smallest_dir_to_update(path: &str) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const NEEDED_SPACE: usize = 30_000_000;

    let input = read_to_string(path).unwrap();
    let dirs = generate_dirs(input.as_str());

    let need_to_clear = NEEDED_SPACE - (TOTAL_SPACE - dirs.get("/").unwrap());

    *dirs
        .values()
        // only care if deleting the dir will free enough space
        .filter(|size| **size >= need_to_clear)
        // get the smallest one
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        let result = find_small_dirs("example.txt");
        assert_eq!(result, 95437)
    }
    #[test]
    fn example_part_two() {
        let result = find_smallest_dir_to_update("example.txt");
        assert_eq!(result, 24933642)
    }
}
