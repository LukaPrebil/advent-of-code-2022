mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day10;
mod day11;

use std::time::Instant;

// https://github.com/drmason13/advent_of_code_traits/blob/d1ade7a19d41f9a509b034bb9de0cb9d631861e6/examples/cli/main.rs
fn main() {
    let day = std::env::args().nth(1)
        .expect(
            "Need a day to know which solution to run, e.g. `cargo run 1` to run day 1 solutions",
        )
        .parse()
        .expect("Unable to parse day, just use a number like `1`");

    let input = std::fs::read_to_string(&find_input(day)).expect("no input available for that day");

    let start = Instant::now();
    match day {
        1 => print_output(
            day,
            day01::solve_1(&day01::parse_input_1(&input)),
            day01::solve_2(&mut day01::parse_input_2(&input)),
        ),
        2 => print_output(
            day,
            day02::solve(&input, day02::get_round_score_1),
            day02::solve(&input, day02::get_round_score_2),
        ),
        3 => print_output(day, day03::solve_1(&input), day03::solve_2(&input)),
        4 => print_output(
            day,
            day04::solve(&input, day04::range_contains_other),
            day04::solve(&input, day04::ranges_overlap),
        ),
        5 => print_output(day, day05::solve(&input, true), day05::solve(&input, false)),
        6 => print_output(day, day06::solve(&input, 4), day06::solve(&input, 14)),
        7 => print_output(day, day07::solve(&input), day07::solve_2(&input)),
        8 => print_output(day, day08::solve(&input), day08::solve_2(&input)),
        10 => print_output(day, day10::solve(&input), 0),
        11 => print_output(day, day11::solve(&input, false), day11::solve(&input, true)),

        x => unimplemented!("no solution available for day {x}"),
    }
    println!("Time: {}us", start.elapsed().as_micros());
}

fn find_input(day: u32) -> String {
    let parent_dir = [
        "advent-of-code-2022/src/input/2022/",
        "src/input/2022/",
        "input/2022/",
    ]
    .iter().find(|path| std::path::Path::new(path).exists())
    .expect("Please run this example from a folder in or above advent-of-code-2022/src/");

    format!("{}/day{:02}.txt", parent_dir, day)
}

fn print_output<Part1SolutionT, Part2SolutionT>(
    day: u32,
    part1: Part1SolutionT,
    part2: Part2SolutionT,
) where
    Part1SolutionT: std::fmt::Display,
    Part2SolutionT: std::fmt::Display,
{
    println!("[Day {}]\n Part 1: {}, Part 2: {}", day, part1, part2)
}
