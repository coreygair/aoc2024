use std::collections::{HashMap, HashSet};

use crate::util::{grid::Grid, position::Position};

type Input = (Grid<char>, HashMap<char, Vec<Position>>);

pub fn parse(input: &str) -> Input {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Position::new(row as i32, col as i32));
            }
        }
    }

    (Grid::from(input), antennas)
}

pub fn part1((grid, antennas): &Input) -> u32 {
    let mut antinodes = HashSet::new();
 
    for (_, antenna_locations) in antennas {
        for (i, a) in antenna_locations.iter().enumerate() {
            for b in &antenna_locations[i+1..] {
                let diff = b - a;

                let x = a - diff;
                if grid.get(x).is_some() {
                    antinodes.insert(x);
                }
                let y = b + diff;
                if grid.get(y).is_some() {
                    antinodes.insert(y);
                }
            }
        }
    }

    antinodes.len() as u32
}

pub fn part2((grid, antennas): &Input) -> u32 {
    let mut antinodes = HashSet::new();
 
    for (_, antenna_locations) in antennas {
        for (i, a) in antenna_locations.iter().enumerate() {
            for b in &antenna_locations[i+1..] {
                let diff = b - a;

                let mut x = a.clone();
                while grid.get(x).is_some() {
                    antinodes.insert(x);
                    x -= diff;
                }

                let mut y = b.clone();
                while grid.get(y).is_some() {
                    antinodes.insert(y);
                    y += diff;
                }
            }
        }
    }

    antinodes.len() as u32
}
