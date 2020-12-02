use aoc2020::day1;
use aoc2020::day2;
use clap::App;
use std::fs;

fn main() {
    let matches = App::new("aoc2020")
        .arg_from_usage("--day1... 'runs the day 1 solution'")
        .arg_from_usage("--day2... 'runs the day 2 solution'")
        .get_matches();

    if matches.is_present("day1") {
        solve_day1()
    }

    if matches.is_present("day2") {
        solve_day2()
    }
}

fn solve_day1() {
    println!("# Day 1\n");

    let input = &fs::read_to_string("input/1.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("## Part 1\n");
    match day1::two_sum(2020, &input) {
        Some(result) => println!(
            "Two numbers: {} {} and their product: {}",
            result.0,
            result.1,
            result.0 * result.1
        ),
        None => println!("No result!"),
    }

    println!("\n## Part 2\n");
    match day1::three_sum(2020, &input) {
        Some(result) => println!(
            "Three numbers: {} {} {} and their product: {}",
            result.0,
            result.1,
            result.2,
            result.0 * result.1 * result.2
        ),
        None => println!("No result!"),
    }
}

fn solve_day2() {
    println!("# Day 2\n");

    let raw_input = &fs::read("input/2.txt").unwrap();
    let input = &raw_input[..raw_input.len() - 1];

    println!("## Part 1\n");
    println!("{}", day2::number_of_valid_part_1_passwords(&input));

    println!("## Part 2\n");
    println!("{}", day2::number_of_valid_part_2_passwords(&input));
}
