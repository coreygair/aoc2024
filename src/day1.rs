use std::collections::HashMap;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    for (i, s) in input.split_whitespace().enumerate() {
        let x = s.parse::<u32>().unwrap();

        if i % 2 == 0 {
            l1.push(x);
        } else {
            l2.push(x);
        }
    }

    (l1, l2)
}

pub fn part1((l1, l2): &Input) -> u32 {
    let mut l1 = l1.clone();
    l1.sort();

    let mut l2 = l2.clone();
    l2.sort();

    let sum_diff: u32 = l1.iter().zip(l2.iter()).map(|(x, y)| x.abs_diff(*y)).sum();

    sum_diff
}

pub fn part2((l1, l2): &Input) -> u32 {
    let mut l2_appearances = HashMap::new();
    for y in l2 {
        *l2_appearances.entry(y).or_insert(0) += 1;
    }

    let mut similarity_score = 0;

    for x in l1 {
        if let Some(count) = l2_appearances.get(&x) {
            similarity_score += x * count;
        }
    }

    similarity_score
}
