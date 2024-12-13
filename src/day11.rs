use std::collections::HashMap;

type Input = Vec<u64>;

pub fn parse(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut stones = input.clone();

    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len());

        for s in stones {
            if s == 0 {
                new_stones.push(1);
            } else if let Some((s1, s2)) = split(s) {
                new_stones.push(s1);
                new_stones.push(s2);
            } else {
                new_stones.push(s * 2024);
            }
        }

        stones = new_stones;
    }

    stones.len() as u64
}

pub fn part2(input: &Input) -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for s in input {
        *stones.entry(*s).or_default() += 1;
    }

    for _ in 0..75 {
        let mut new_stones = HashMap::with_capacity(stones.len());

        for (s, count) in stones {
            if s == 0 {
                *new_stones.entry(1).or_default() += count;
            } else if let Some((s1, s2)) = split(s) {
                *new_stones.entry(s1).or_default() += count;
                *new_stones.entry(s2).or_default() += count;
            } else {
                *new_stones.entry(s * 2024).or_default() += count;
            }
        }

        stones = new_stones;
    }

    stones.into_iter().map(|(_, count)| count).sum()
}

fn split(stone: u64) -> Option<(u64, u64)> {
    let n_digits = stone.ilog10() + 1;

    if n_digits % 2 == 1 {
        return None;
    }

    let d = 10u64.pow(n_digits / 2) as u64;
    let s1 = stone / d;
    Some((s1, stone - (s1 * d)))
}
