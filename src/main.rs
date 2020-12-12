use aoc2020::{day1, day2, day3, day4, day5};
use clap::App;
use std::convert::TryInto;
use std::fs;

fn main() {
    let matches = App::new("aoc2020")
        .arg_from_usage("--day1... 'runs the day 1 solution'")
        .arg_from_usage("--day2... 'runs the day 2 solution'")
        .arg_from_usage("--day3... 'runs the day 3 solution'")
        .arg_from_usage("--day4... 'runs the day 4 solution'")
        .arg_from_usage("--day5... 'runs the day 5 solution'")
        .get_matches();

    if matches.is_present("day1") {
        solve_day1()
    }

    if matches.is_present("day2") {
        solve_day2()
    }

    if matches.is_present("day3") {
        solve_day3()
    }

    if matches.is_present("day4") {
        solve_day4()
    }

    if matches.is_present("day5") {
        solve_day5()
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

    let input = &fs::read("input/2.txt").unwrap();

    println!("## Part 1\n");
    println!("{}", day2::number_of_valid_part_1_passwords(&input));

    println!("## Part 2\n");
    println!("{}", day2::number_of_valid_part_2_passwords(&input));
}

fn solve_day3() {
    println!("# Day 3\n");

    let input = &fs::read("input/3.txt").unwrap();

    println!("## Part 1\n");
    println!("{}", day3::part_1(&input));
    println!();

    println!("## Part 2\n");
    println!("{}", day3::part_2(&input));
}

fn solve_day4() {
    println!("# Day 4\n");

    let input = &fs::read_to_string("input/4.txt").unwrap();

    println!("## Part 1\n");
    println!("{}", day4::number_of_passports_with_valid_fields(&input));

    println!("## Part 2\n");
    println!(
        "{}",
        day4::number_of_passports_with_valid_fields_and_values(&input)
    );
}

fn solve_day5() {
    println!("# Day 5\n");

    let input = &fs::read_to_string("input/5.txt").unwrap();

    println!("## Part 1\n");
    let max = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| day5::seat_id(s.as_bytes()))
        .max()
        .unwrap();
    println!("{}", max);

    println!("## Part 2\n");
    let mut exists: [[bool; 8]; 128] = [[false; 8]; 128];
    let seats = input.split("\n").filter(|s| !s.is_empty()).map(|s| {
        day5::locate_seat(day5::parse(s.as_bytes().try_into().unwrap()))
    });

    for (row, col) in seats {
        exists[row][col] = true
    }

    for (row_id, row) in exists.iter().enumerate() {
        for (seat_id, seat) in row.iter().enumerate() {
            if !seat {
                println!(
                    "Empty seat: ({}, {}) - id: {}",
                    row_id,
                    seat_id,
                    row_id * 8 + seat_id
                );
            }
        }
    }
}
