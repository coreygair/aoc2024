use std::collections::HashSet;

use crate::util::{grid::Grid, position::{Direction, Position}};

type Input = (Grid<MapCell>, Position, Direction);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MapCell {
    Empty,
    Obstruction,
}

impl From<char> for MapCell {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => MapCell::Empty,
            '#' => MapCell::Obstruction,
            _ => unreachable!(),
        }
    }
}

pub fn parse(input: &str) -> Input {
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(row, l)| {
            l.chars().enumerate().find_map(|(col, c)| {
                Some(match c {
                    '^' => Position::new(row as i32, col as i32),
                    _ => return None,
                })
            })
        })
        .unwrap();

    (input.into(), start_pos, Direction::Up)
}

pub fn part1((map, start_pos, start_dir): &Input) -> u32 {
    visited(map, *start_pos, *start_dir).len() as u32
}

pub fn part2((map, start_pos, start_dir): &Input) -> u32 {
    let mut map = map.clone();

    let mut visited = visited(&map, *start_pos, *start_dir);
    visited.remove(start_pos);
    visited
        .iter()
        .map(|pos| {
            map.set(*pos, MapCell::Obstruction);
            let has_loop = contains_loop(&map, *start_pos, *start_dir);
            map.set(*pos, MapCell::Empty);

            if has_loop {
                1
            } else {
                0
            }
        })
        .sum()
}

fn visited(map: &Grid<MapCell>, start_pos: Position, start_dir: Direction) -> HashSet<Position> {
    let mut visited = HashSet::new();

    let mut pos = start_pos.clone();
    visited.insert(pos);

    let mut dir = start_dir.clone();
    loop {
        let new_pos = pos.moved_in(dir);
        match map.get(new_pos) {
            None => break,
            Some(MapCell::Empty) => {
                visited.insert(new_pos);
                pos = new_pos;
            }
            Some(MapCell::Obstruction) => {
                dir = dir.rotated_clockwise();
            }
        }
    }

    visited
}

fn contains_loop(map: &Grid<MapCell>, start_pos: Position, start_dir: Direction) -> bool {
    let mut visited = HashSet::new();

    let mut pos = start_pos.clone();
    let mut dir = start_dir.clone();
    visited.insert((pos, dir));

    loop {
        let new_pos = pos.moved_in(dir);
        match map.get(new_pos) {
            None => break,
            Some(MapCell::Empty) => {
                if !visited.insert((new_pos, dir)) {
                    return true;
                }
                pos = new_pos;
            }
            Some(MapCell::Obstruction) => {
                dir = dir.rotated_clockwise();
            }
        }
    }

    false
}
