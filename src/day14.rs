use regex::Regex;

type Input = Vec<Robot>;

pub fn parse(input: &str) -> Input {
    Regex::new("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|c| Robot {
            p: (
                c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            v: (
                c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|r| r.position_after(100))
        .fold([0u32; 4], |mut acc, (x, y)| {
            if x < COLS / 2 && y < ROWS / 2 {
                acc[0] += 1;
            } else if x < COLS / 2 && y > ROWS / 2 {
                acc[1] += 1;
            } else if x > COLS / 2 && y < ROWS / 2 {
                acc[2] += 1;
            } else if x > COLS / 2 && y > ROWS / 2 {
                acc[3] += 1;
            }
            acc
        })
        .iter()
        .product()
}

pub fn part2(input: &Input) -> u64 {
    // Guess the image will be in the center of grid,
    // so avg distance to center will be lowest.
    // May repeat many times but this should give the first occurence.
    (0..COLS * ROWS)
        .into_iter()
        .map(|s| {
            input
                .iter()
                .map(|r| {
                    let (x, y) = r.position_after(s);
                    (COLS / 2).abs_diff(x) + (ROWS / 2).abs_diff(y)
                })
                .sum::<u32>()
        })
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0 as u64
}

const ROWS: i32 = 103;
const COLS: i32 = 101;

pub struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn position_after(&self, s: i32) -> (i32, i32) {
        (
            // Rust % is remainder not modulo so add multiple of rows/cols to prevent negatives.
            (self.p.0 + (s * self.v.0) + (s * COLS)) % COLS,
            (self.p.1 + (s * self.v.1) + (s * ROWS)) % ROWS,
        )
    }
}
