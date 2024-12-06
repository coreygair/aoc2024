use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Instant,
};

mod util {
    pub mod grid;
}

// Day 0 = template.
#[allow(unused)]
mod day0;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("--- AoC 2024! ---\n");

    let start = Instant::now();

    for solution in solutions() {
        let input = read_to_string(solution.input).unwrap();

        let day_start = Instant::now();
        let (part1, part2) = (solution.run)(input);
        let duration = day_start.elapsed();

        println!(
            "{} completed in {:?}. Part 1: {}. Part 2: {}.",
            solution.day, duration, part1, part2
        );
    }

    let duration = start.elapsed();

    println!("\nCompleted {} days in {:?}.", solutions().len(), duration);
}

struct Solution {
    day: &'static str,
    input: PathBuf,
    run: fn(String) -> (String, String),
}

macro_rules! solution {
    ($day:tt) => {{
        let day = stringify!($day);
        let input = Path::new("inputs").join(day).with_extension("txt");
        let run = |data: String| {
            use $day::*;

            let input = parse(&data);
            (part1(&input).to_string(), part2(&input).to_string())
        };

        Solution { day, input, run }
    }};
}

fn solutions() -> Vec<Solution> {
    vec![
        solution!(day1),
        solution!(day2),
        solution!(day3),
        solution!(day4),
        solution!(day5),
        solution!(day6),
    ]
}
