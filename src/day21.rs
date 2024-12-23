use std::{collections::HashMap, fmt::Write};

use crate::util::position::{Direction, Position};

type Input = Vec<Vec<NumpadButton>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| NumpadButton::from(c)).collect())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    score_input(input, 2)
}

pub fn part2(input: &Input) -> u64 {
    score_input(input, 25)
}

/// All of the code below this point is very ugly,
/// but it works and is fast so idc :)
fn score_input(input: &Input, depth: u64) -> u64 {
    let mut cache = HashMap::new();

    input
        .iter()
        .map(|x| {
            let mut s = 0;
            let mut curr = NumpadButton::A;
            for b in x {
                let mut curr2 = DPadButton::A;
                for b2 in numpad_moves_to_press(curr, *b) {
                    s += score(&mut cache, curr2, b2, depth);
                    curr2 = b2;
                }

                curr = *b;
            }

            let mut n = 0;
            for b in x.iter() {
                if let NumpadButton::Number(x) = b {
                    n = n * 10 + x;
                }
            }

            s * n
        })
        .sum::<u64>()
}

fn score(
    cache: &mut HashMap<(DPadButton, DPadButton, u64), u64>,
    from: DPadButton,
    to: DPadButton,
    depth: u64,
) -> u64 {
    if let Some(score) = cache.get(&(from, to, depth)) {
        return *score;
    }

    let moves = dpad_moves_to_press(from, to);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut calculated_score = 0;
    let mut curr = DPadButton::A;
    for d in moves {
        calculated_score += score(cache, curr, d, depth - 1);
        curr = d;
    }

    cache.insert((from, to, depth), calculated_score);
    calculated_score
}

fn numpad_moves_to_press(from: NumpadButton, to: NumpadButton) -> Vec<DPadButton> {
    let mut ds: Vec<DPadButton> = numpad_moves(from, to)
        .into_iter()
        .map(|d| d.into())
        .collect();
    ds.push(DPadButton::A);
    ds
}

// Heuristic seems to be:
// - Do all of one directions moves and then the other, don't zig-zag.
// - Prefer < then v then ^ then > (except when that would put you off the buttons)
fn numpad_moves(from: NumpadButton, to: NumpadButton) -> Vec<Direction> {
    use NumpadButton::*;

    match (from, to) {
        (A, A) => vec![],
        (A, Number(0)) => vec![Direction::Left],
        (A, Number(2)) => vec![Direction::Left, Direction::Up],
        (A, Number(5)) => vec![Direction::Left, Direction::Up, Direction::Up],
        (A, Number(8)) => vec![Direction::Left, Direction::Up, Direction::Up, Direction::Up],
        (A, Number(n)) => {
            let Position { row: dy, col: dx } = position_of(n) - Position::new(0, 0);

            let mut out = vec![Direction::Up; dy.abs() as usize];

            match dx {
                0 => {}
                -2 => {
                    out.extend(vec![Direction::Left; 2]);
                }
                _ => panic!(),
            }

            out
        }

        (Number(n), Number(m)) if n == m => vec![],

        (Number(0), A) => vec![Direction::Right],
        (Number(0), Number(n)) => {
            let Position { row: dy, col: dx } = position_of(n) - Position::new(0, -1);

            let mut out = vec![Direction::Up; dy.abs() as usize];

            match dx {
                -1 => out.push(Direction::Left),
                0 => {}
                1 => out.push(Direction::Right),
                _ => panic!(),
            }

            out
        }

        (Number(n), A) => {
            let p = position_of(n);
            let Position { row: dy, col: dx } = Position::new(0, 0) - position_of(n);

            if p.col == -2 {
                let mut out = vec![Direction::Right; dx.abs() as usize];
                out.extend(vec![Direction::Down; dy.abs() as usize]);
                return out;
            }

            let mut out = vec![Direction::Down; dy.abs() as usize];
            out.extend(vec![Direction::Right; dx.abs() as usize]);
            out
        }
        (Number(n), Number(0)) => {
            let Position { row: dy, col: dx } = Position::new(0, -1) - position_of(n);

            if dx > 0 {
                let mut out = vec![Direction::Right; dx.abs() as usize];
                out.extend(vec![Direction::Down; dy.abs() as usize]);
                return out;
            }

            let mut out = vec![Direction::Left; dx.abs() as usize];
            out.extend(vec![Direction::Down; dy.abs() as usize]);

            out
        }
        (Number(n), Number(n2)) => {
            let Position { row: dy, col: dx } = position_of(n2) - position_of(n);

            let mut out = Vec::with_capacity(4);
            if dx < 0 {
                out.extend(vec![Direction::Left; dx.abs() as usize]);
            }
            if dy > 0 {
                out.extend(vec![Direction::Down; dy.abs() as usize]);
            }
            if dy < 0 {
                out.extend(vec![Direction::Up; dy.abs() as usize]);
            }
            if dx > 0 {
                out.extend(vec![Direction::Right; dx.abs() as usize]);
            }

            out
        }
    }
}

#[derive(Clone, Copy)]
pub enum NumpadButton {
    Number(u64),
    A,
}

impl From<char> for NumpadButton {
    fn from(value: char) -> Self {
        if value == 'A' {
            return NumpadButton::A;
        }

        let value = value.to_digit(10).unwrap() as u64;

        if value > 9 {
            unreachable!()
        }

        NumpadButton::Number(value)
    }
}

fn dpad_moves_to_press(from: DPadButton, to: DPadButton) -> Vec<DPadButton> {
    let mut ds: Vec<DPadButton> = dpad_moves(from, to).into_iter().map(|d| d.into()).collect();
    ds.push(DPadButton::A);
    ds
}

// Same heuristic as numpad_moves
fn dpad_moves(from: DPadButton, to: DPadButton) -> Vec<Direction> {
    match (from, to) {
        (DPadButton::A, DPadButton::Direction(Direction::Up)) => vec![Direction::Left],
        (DPadButton::A, DPadButton::Direction(Direction::Right)) => vec![Direction::Down],
        (DPadButton::A, DPadButton::Direction(Direction::Down)) => {
            vec![Direction::Left, Direction::Down]
        }
        (DPadButton::A, DPadButton::Direction(Direction::Left)) => {
            vec![Direction::Down, Direction::Left, Direction::Left]
        }

        (DPadButton::Direction(Direction::Up), DPadButton::A) => {
            vec![Direction::Right]
        }
        (DPadButton::Direction(Direction::Up), DPadButton::Direction(Direction::Down)) => {
            vec![Direction::Down]
        }
        (DPadButton::Direction(Direction::Up), DPadButton::Direction(Direction::Left)) => {
            vec![Direction::Down, Direction::Left]
        }
        (DPadButton::Direction(Direction::Up), DPadButton::Direction(Direction::Right)) => {
            vec![Direction::Down, Direction::Right]
        }

        (DPadButton::Direction(Direction::Down), DPadButton::A) => {
            vec![Direction::Up, Direction::Right]
        }
        (DPadButton::Direction(Direction::Down), DPadButton::Direction(Direction::Up)) => {
            vec![Direction::Up]
        }
        (DPadButton::Direction(Direction::Down), DPadButton::Direction(Direction::Left)) => {
            vec![Direction::Left]
        }
        (DPadButton::Direction(Direction::Down), DPadButton::Direction(Direction::Right)) => {
            vec![Direction::Right]
        }

        (DPadButton::Direction(Direction::Left), DPadButton::A) => {
            vec![Direction::Right, Direction::Right, Direction::Up]
        }
        (DPadButton::Direction(Direction::Left), DPadButton::Direction(Direction::Up)) => {
            vec![Direction::Right, Direction::Up]
        }
        (DPadButton::Direction(Direction::Left), DPadButton::Direction(Direction::Down)) => {
            vec![Direction::Right]
        }
        (DPadButton::Direction(Direction::Left), DPadButton::Direction(Direction::Right)) => {
            vec![Direction::Right, Direction::Right]
        }

        (DPadButton::Direction(Direction::Right), DPadButton::A) => {
            vec![Direction::Up]
        }
        (DPadButton::Direction(Direction::Right), DPadButton::Direction(Direction::Up)) => {
            vec![Direction::Left, Direction::Up]
        }
        (DPadButton::Direction(Direction::Right), DPadButton::Direction(Direction::Down)) => {
            vec![Direction::Left]
        }
        (DPadButton::Direction(Direction::Right), DPadButton::Direction(Direction::Left)) => {
            vec![Direction::Left, Direction::Left]
        }

        (x, _y) => {
            assert!(matches!(x, _y));
            vec![]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DPadButton {
    Direction(Direction),
    A,
}

impl From<Direction> for DPadButton {
    fn from(value: Direction) -> Self {
        DPadButton::Direction(value)
    }
}

impl std::fmt::Debug for DPadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DPadButton::A => f.write_char('A'),
            DPadButton::Direction(d) => f.write_char(d.to_arrow()),
        }
    }
}

fn position_of(n: u64) -> Position {
    match n {
        0 => Position::new(0, -1),

        1 => Position::new(-1, -2),
        2 => Position::new(-1, -1),
        3 => Position::new(-1, 0),

        4 => Position::new(-2, -2),
        5 => Position::new(-2, -1),
        6 => Position::new(-2, 0),

        7 => Position::new(-3, -2),
        8 => Position::new(-3, -1),
        9 => Position::new(-3, 0),

        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part1(&parse("029A")), 1972);
    }
}
