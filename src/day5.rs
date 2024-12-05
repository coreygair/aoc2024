use std::collections::{HashMap, HashSet};

type Input = (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>);

pub fn parse(input: &str) -> Input {
    let mut page_by_before_set: HashMap<u32, HashSet<u32>> = HashMap::new();

    let before_rules: Vec<(u32, u32)> = input
        .lines()
        .take_while(|l| *l != "")
        .map(|l| {
            let mut ns = l.split("|").map(|s| s.parse::<u32>().unwrap());
            (ns.next().unwrap(), ns.next().unwrap())
        })
        .collect();
    for (before, after) in before_rules {
        page_by_before_set.entry(before).or_default().insert(after);
    }

    let updates = input
        .lines()
        .skip_while(|l| *l != "")
        .skip(1)
        .map(|l| l.split(",").map(|s| s.parse::<u32>().unwrap()).collect())
        .collect();

    (page_by_before_set, updates)
}

pub fn part1((page_by_before_set, updates): &Input) -> u32 {
    updates
        .iter()
        .map(|update| {
            if is_correct(page_by_before_set, update) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

pub fn part2((page_by_before_set, updates): &Input) -> u32 {
    updates
        .iter()
        .filter(|update| !is_correct(page_by_before_set, update))
        .cloned()
        .map(|mut update| {
            let mut done = false;
            while !done {
                done = true;

                'outer: for i in 0..update.len() {
                    let n = update[i];
                    for j in i + 1..update.len() {
                        let m = update[j];
                        if let Some(before_set) = page_by_before_set.get(&m) {
                            if before_set.contains(&n) {
                                (update[i], update[j]) = (m, n);
                                done = false;
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            update[update.len() / 2]
        })
        .sum()
}

fn is_correct(page_by_before_set: &HashMap<u32, HashSet<u32>>, update: &Vec<u32>) -> bool {
    for (i, n) in update.iter().enumerate() {
        if let Some(before_set) = page_by_before_set.get(n) {
            for m in &update[..i] {
                if before_set.contains(m) {
                    return false;
                }
            }
            for m in &update[i + 1..] {
                if !before_set.contains(m) {
                    return false;
                }
            }
        }
    }

    true
}
