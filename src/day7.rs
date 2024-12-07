type Input = Vec<(u64, Vec<u64>)>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|l| {
        let mut parts = l.split(": ");
        let goal = parts.next().unwrap().parse::<u64>().unwrap();
        let operands = parts.next().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

        (goal, operands)
    }).collect()
}

pub fn part1(input: &Input) -> u64 {
    input.iter().filter(|(goal, operands)| is_equation_possibly_true(false, *goal, operands)).map(|(goal, _)| goal).sum()
}

pub fn part2(input: &Input) -> u64 {
    input.iter().filter(|(goal, operands)| is_equation_possibly_true(true, *goal, operands)).map(|(goal, _)| goal).sum()
}

fn is_equation_possibly_true(with_concat: bool, goal: u64, operands: &Vec<u64>) -> bool {
    let mut goals = Vec::new();
    goals.push(goal);

    for operand in operands.iter().rev() {
        let mut new_goals = Vec::new();
        for goal in goals.iter() {
            // +
            if goal >= operand {
                new_goals.push(goal - operand);
            }

            // *
            if goal % operand == 0 {
                new_goals.push(goal / operand);
            }

            // ||
            if with_concat {
                let multiplier = 10u64.pow(operand.ilog10() + 1);
                if (goal / multiplier) * multiplier + operand == *goal {
                    new_goals.push(goal / (10u64.pow(operand.ilog10() + 1)));
                }
            }
        }

        goals = new_goals;
    }

    goals.contains(&0)
}
