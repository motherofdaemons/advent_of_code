
fn main() {
    println!("Top elf: {}", day_1::nth_most_calories("input.txt", 1).unwrap());
    println!("Top 3 elves: {}", day_1::nth_most_calories("input.txt", 3).unwrap());
}