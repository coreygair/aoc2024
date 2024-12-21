use std::collections::HashMap;

use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

type Input = (Grid<Cell>, Vec<Position>, HashMap<Position, usize>);

pub fn parse(input: &str) -> Input {
    let grid: Grid<Cell> = input.into();

    let start = grid.find(Cell::Start).unwrap();
    let end = grid.find(Cell::End).unwrap();

    let mut path = Vec::new();
    let mut distance_to_end = HashMap::new();

    let mut d = Direction::ALL
        .iter()
        .cloned()
        .find(|d| matches!(grid.get(end.moved_in(*d)), Some(Cell::Space)))
        .unwrap();

    let mut p = end;
    loop {
        distance_to_end.insert(p, path.len());
        path.push(p);

        if p == start {
            break;
        }

        for d2 in Direction::ALL {
            if d2 == d.reversed() {
                continue;
            }

            if matches!(grid.get(p.moved_in(d2)), Some(Cell::Space)) {
                d = d2;
                break;
            }
        }
        p = p.moved_in(d);
    }

    (grid, path, distance_to_end)
}

pub fn part1((grid, path, distance_to_end): &Input) -> u64 {
    let mut count = 0;

    for (to_end, p) in path[100..].iter().enumerate() {
        for d in Direction::ALL {
            if matches!(grid.get(p.moved_in(d)), Some(Cell::Wall)) {
                if let Some(to_end2) = distance_to_end.get(&p.moved_in(d).moved_in(d)) {
                    if *to_end2 + 1 < to_end {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn part2((_, path, _): &Input) -> u64 {
    let mut count = 0;

    for (to_end, p) in path[100..].iter().enumerate() {
        for (to_end2, p2) in path[..to_end].iter().enumerate() {
            let m = p.row.abs_diff(p2.row) + p.col.abs_diff(p2.col);
            if to_end2 + (m as usize) - 1 < to_end && m <= 20 {
                count += 1;
            }
        }
    }

    count
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Space,
    Wall,
    Start,
    End,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Space,
            '#' => Cell::Wall,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => unreachable!(),
        }
    }
}
