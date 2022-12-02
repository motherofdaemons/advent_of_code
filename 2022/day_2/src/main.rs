fn main() {
    println!("Part 1: {}", day_2::rps_tournament("input.txt", &day_2::RPSStrategy::Move).unwrap());
    println!("Part 2: {}", day_2::rps_tournament("input.txt", &day_2::RPSStrategy::WinLoseTie).unwrap());
}