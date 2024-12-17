use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Instant,
};

mod util {
    pub mod grid;
    pub mod position;
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
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

fn main() {
    println!("--- AoC 2024! ---\n");

    let start = Instant::now();

    for solution in solutions() {
        let input = match read_to_string(solution.input) {
            Ok(s) => s,
            Err(e) => {
                println!("{} failed to load input: {}", solution.day, e);
                continue;
            }
        };

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
        //solution!(day6),
        solution!(day7),
        solution!(day8),
        solution!(day9),
        solution!(day10),
        solution!(day11),
        solution!(day12),
        solution!(day13),
        solution!(day14),
        solution!(day15),
        solution!(day16),
        solution!(day17),
    ]
}
