use std::collections::HashMap;

type Input<'a> = (Vec<&'a str>, Vec<&'a str>);

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let patterns = lines.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = lines.skip(1).collect();

    (patterns, designs)
}

pub fn part1((patterns, designs): &Input) -> u64 {
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter(|d| is_design_possible(&mut memo, patterns, d))
        .count() as u64
}

fn is_design_possible<'a>(
    memo: &mut HashMap<&'a str, bool>,
    patterns: &[&str],
    design: &'a str,
) -> bool {
    if let Some(possible) = memo.get(design) {
        return *possible;
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if rest == "" || is_design_possible(memo, patterns, rest) {
                memo.insert(design, true);
                return true;
            }
        }
    }

    memo.insert(design, false);
    return false;
}

pub fn part2((patterns, designs): &Input) -> u64 {
    let mut memo = HashMap::new();

    designs
        .iter()
        .map(|d| count_ways_possible(&mut memo, patterns, d))
        .sum()
}

fn count_ways_possible<'a>(
    memo: &mut HashMap<&'a str, u64>,
    patterns: &[&str],
    design: &'a str,
) -> u64 {
    if let Some(count) = memo.get(design) {
        return *count;
    }

    let mut count = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if rest == "" {
                count += 1;
            } else {
                count += count_ways_possible(memo, patterns, rest);
            }
        }
    }

    memo.insert(design, count);
    return count;
}
