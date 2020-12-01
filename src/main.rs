use aoc2020::day1::two_sum;
use std::fs;

fn main() {
    let input = &fs::read_to_string("input/1.txt").unwrap();
    match two_sum(
        2020,
        &input
            .trim()
            .split('\n')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    ) {
        Some(result) => println!(
            "Two numbers: {} {} and their product: {}",
            result.0,
            result.1,
            result.0 * result.1
        ),
        None => println!("No result!"),
    }
}
