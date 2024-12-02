type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().filter(|r| is_safe(*r)).count() as u32
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .filter(|r| {
            let report = *r;

            if is_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                if is_safe(&report) {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

fn is_safe(report: &Vec<u32>) -> bool {
    let decreasing = report[0] > report[1];

    for window in report.windows(2) {
        let [i, j, ..] = window else {
            break;
        };

        if decreasing {
            if *i <= *j || *i - *j > 3 {
                return false;
            }
        } else {
            if *j <= *i || *j - *i > 3 {
                return false;
            }
        }
    }

    true
}
