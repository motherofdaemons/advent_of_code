use std::error::Error;

// Take in a path to the input file that is double newline seperated elves
pub fn nth_most_calories(path: &str, n: usize) -> Result<i32, Box<dyn Error>> {
    let input = std::fs::read_to_string(path)?;
    let mut calorie_sums = input
        .split("\n\n") //split the entire string into individual elves
        .map(|sp| {
            sp.split_whitespace() //split the what the elves are carrying into each item
                .filter_map(|x| x.parse().ok()) //filter out and thing that doesn't convert to i32 and convert it
                .collect::<Vec<i32>>() //then we gather all of them and sum them up
                .iter()
                .sum()
        })
        .collect::<Vec<i32>>();
    //sort the list and then we have to reverse it to get it sorted largest to smallest
    //we can then take the top n and sum them
    calorie_sums.sort();
    Ok(calorie_sums.iter().rev().take(n).sum())
}

mod tests {
    use super::*;

    #[test]
    fn example_most_calories() {
        let result = nth_most_calories("example.txt", 1).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn example_top_three_calories() {
        let result = nth_most_calories("example.txt", 3).unwrap();
        assert_eq!(result, 45000);
    }
}
