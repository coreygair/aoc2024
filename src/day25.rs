type Pins = [u8; 5];

type Input = (Vec<Pins>, Vec<Pins>);

pub fn parse(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let mut acc = Pins::default();
    let mut row = 0;
    let mut is_key = false;
    for l in input.lines() {
        if l == "" {
            row = 0;
            acc = Pins::default();
            continue;
        }

        if row == 0 {
            is_key = l == ".....";
            row += 1;
            continue;
        }

        if row == 6 {
            if is_key {
                keys.push(acc);
            } else {
                locks.push(acc);
            }
            continue;
        }

        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                acc[i] += 1;
            }
        }

        row += 1;
    }

    (keys, locks)
}

pub fn part1((keys, locks): &Input) -> u64 {
    keys.iter().map(|k| {
        locks.iter().map(|l| {
            for i in 0..5 {
                if k[i] + l[i] > 5 {
                    return 0;
                }
            }
            1
        }).sum::<u64>()
    }).sum()
}

pub fn part2(_input: &Input) -> u64 {
    0
}

