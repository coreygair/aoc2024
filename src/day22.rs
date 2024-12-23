use std::collections::{HashMap, HashSet};

type Input = Vec<u64>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .cloned()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret
        })
        .sum::<u64>()
}

pub fn part2(input: &Input) -> u64 {
    // Maps 4-delta -> sum of price at first occurrence per input.
    let mut delta_to_total_price = HashMap::new();

    for mut secret in input.iter().cloned() {
        let mut seen = HashSet::new();

        let mut curr_deltas = [0; 4];

        for i in 0..2000 {
            let next_secret = next(secret);

            let delta = last_digit(next_secret) - last_digit(secret);
            curr_deltas.rotate_left(1);
            curr_deltas[3] = delta;

            secret = next_secret;

            if i > 2 {
                // Now have a full list of last 4 price changes.
                if !seen.contains(&curr_deltas) {
                    seen.insert(curr_deltas);

                    *delta_to_total_price.entry(curr_deltas).or_insert(0u64) +=
                        last_digit(secret) as u64;
                }
            }
        }
    }

    *delta_to_total_price
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .1
}

fn next(a: u64) -> u64 {
    let a = prune(mix(a, a * 64));
    let a = prune(mix(a, a / 32));
    prune(mix(a, a * 2048))
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn last_digit(a: u64) -> i8 {
    (a % 10) as i8
}
