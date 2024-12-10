use std::collections::{HashMap, HashSet};

use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

type Input = Grid<Height>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Height(pub u8);

impl From<char> for Height {
    fn from(value: char) -> Self {
        Height(value.to_digit(10).unwrap() as u8)
    }
}

impl std::ops::Add<u8> for Height {
    type Output = Height;

    fn add(self, rhs: u8) -> Self::Output {
        Height(self.0 + rhs)
    }
}

pub fn parse(input: &str) -> Input {
    Grid::from(input)
}

pub fn part1(input: &Input) -> u32 {
    let mut pos_to_9s = HashMap::new();

    input
        .iter()
        .map(|(position, height)| {
            if height == Height(0) {
                dfs_unique(&mut pos_to_9s, input, position, height).len() as u32
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let mut pos_to_trails = HashMap::new();

    input
        .iter()
        .map(|(position, height)| {
            if height == Height(0) {
                dfs_count(&mut pos_to_trails, input, position, height)
            } else {
                0
            }
        })
        .sum()
}

pub fn dfs_unique(
    pos_to_9s: &mut HashMap<Position, HashSet<Position>>,
    map: &Grid<Height>,
    position: Position,
    height: Height,
) -> HashSet<Position> {
    let mut result = HashSet::new();

    if let Some(n_trails) = pos_to_9s.get(&position) {
        return n_trails.clone();
    }

    for d in Direction::ALL {
        let position = position.moved_in(d);

        if let Some(h2) = map.get(position) {
            if *h2 == height + 1 {
                if *h2 == Height(9) {
                    result.insert(position);
                } else {
                    result.extend(dfs_unique(pos_to_9s, map, position, *h2));
                }
            }
        }
    }

    pos_to_9s.insert(position, result.clone());
    result
}

pub fn dfs_count(
    pos_to_trails: &mut HashMap<Position, u32>,
    map: &Grid<Height>,
    position: Position,
    height: Height,
) -> u32 {
    if let Some(n_trails) = pos_to_trails.get(&position) {
        return *n_trails;
    }

    let mut result = 0;
    for d in Direction::ALL {
        let position = position.moved_in(d);

        if let Some(h2) = map.get(position) {
            if *h2 == height + 1 {
                if *h2 == Height(9) {
                    result += 1;
                } else {
                    result += dfs_count(pos_to_trails, map, position, *h2);
                }
            }
        }
    }

    pos_to_trails.insert(position, result.clone());
    result
}
