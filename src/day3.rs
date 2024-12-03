use regex::Regex;

// (x, y, enabled) -> mul(x,y) with enabled flag.
type Input = Vec<(u32, u32, bool)>;

pub fn parse(input: &str) -> Input {
    let r = Regex::new("mul\\(([0-9]+),([0-9]+)\\)|do\\(\\)|don't\\(\\)").unwrap();

    let mut enabled = true;
    r.captures_iter(input)
        .filter_map(|captures| match captures.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                None
            }
            "don't()" => {
                enabled = false;
                None
            }
            _ => {
                let x = captures.get(1).unwrap().as_str();
                let y = captures.get(2).unwrap().as_str();
                Some((
                    x.parse::<u32>().unwrap(),
                    y.parse::<u32>().unwrap(),
                    enabled,
                ))
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|(x, y, _)| x * y).sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|(x, y, enabled)| if *enabled { x * y } else { 0 })
        .sum()
}
