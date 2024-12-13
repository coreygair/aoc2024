use regex::Regex;

pub struct Machine {
    a_x: i64,
    a_y: i64,

    b_x: i64,
    b_y: i64,

    p_x: i64,
    p_y: i64,
}

type Input = Vec<Machine>;

pub fn parse(input: &str) -> Input {
    let mut machines = Vec::new();

    let a = Regex::new("Button A: X\\+(\\d+), Y\\+(\\d+)").unwrap();
    let b = Regex::new("Button B: X\\+(\\d+), Y\\+(\\d+)").unwrap();
    let prize = Regex::new("Prize: X=(\\d+), Y=(\\d+)").unwrap();

    let a_matches = a.captures_iter(input);
    let b_matches = b.captures_iter(input);
    let prize_matches = prize.captures_iter(input);

    for ((a, b), p) in a_matches.zip(b_matches).zip(prize_matches) {
        let a_x = a.get(1).unwrap().as_str().parse::<u64>().unwrap() as i64;
        let a_y = a.get(2).unwrap().as_str().parse::<u64>().unwrap() as i64;

        let b_x = b.get(1).unwrap().as_str().parse::<u64>().unwrap() as i64;
        let b_y = b.get(2).unwrap().as_str().parse::<u64>().unwrap() as i64;

        let p_x = p.get(1).unwrap().as_str().parse::<u64>().unwrap() as i64;
        let p_y = p.get(2).unwrap().as_str().parse::<u64>().unwrap() as i64;

        machines.push(Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            p_x,
            p_y,
        });
    }

    machines
}

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|m| {
            let inv_det = (m.a_x * m.b_y) - (m.b_x * m.a_y);
            let a = ((m.b_y * m.p_x) - (m.b_x * m.p_y)) / inv_det;
            let b = ((m.a_x * m.p_y) - (m.a_y * m.p_x)) / inv_det;

            if a * m.a_x + b * m.b_x == m.p_x && a * m.a_y + b * m.b_y == m.p_y {
                3 * a as u64 + b as u64
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    let input = input
        .iter()
        .map(|m| Machine {
            p_x: m.p_x + 10000000000000,
            p_y: m.p_y + 10000000000000,
            ..*m
        })
        .collect();

    part1(&input)
}
