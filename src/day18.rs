use std::collections::HashSet;

use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

type Input = Vec<Position>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let row = parts.next().unwrap().parse::<i32>().unwrap();
            let col = parts.next().unwrap().parse::<i32>().unwrap();
            Position::new(row, col)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut map = Grid::new(71, 71, Cell::Space);

    for p in input.iter().take(1024) {
        map.set(*p, Cell::Corrupted);
    }

    shortest_path(&map).unwrap()
}

pub fn part2(input: &Input) -> String {
    let mut map = Grid::new(71, 71, Cell::Space);

    let mut prev_applied = 0;

    let mut lower = 0;
    let mut upper = input.len();

    loop {
        let current_applied = (upper + lower) / 2;

        // Avoids cloning + applying from 0 on each iter.
        if prev_applied < current_applied {
            for p in input[prev_applied..=current_applied].iter() {
                map.set(*p, Cell::Corrupted);
            }
        } else {
            for p in input[current_applied + 1..=prev_applied].iter().rev() {
                // By inspection the input does not contain duplicates so this is safe :)
                map.set(*p, Cell::Space);
            }
        }

        match shortest_path(&map) {
            None => {
                upper = current_applied;
            }
            Some(_) => {
                lower = current_applied + 1;

                if lower == upper {
                    return format!("{},{}", input[lower].row, input[lower].col);
                }
            }
        }

        prev_applied = current_applied;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Space,
    Corrupted,
}

struct Entry {
    pos: Position,
    distance: u64,
}

fn shortest_path(map: &Grid<Cell>) -> Option<u64> {
    let mut visited = HashSet::new();

    let mut q = Vec::new();
    q.push(Entry {
        pos: Position::new(0, 0),
        distance: 0,
    });

    while let Some(Entry { pos, distance }) = q.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if pos == Position::new(70, 70) {
            return Some(distance);
        }

        for d in Direction::ALL {
            let new_pos = pos.moved_in(d);
            let Some(new_cell) = map.get(new_pos) else {
                continue;
            };
            if *new_cell == Cell::Corrupted {
                continue;
            }

            q.push(Entry {
                pos: new_pos,
                distance: distance + 1,
            });
        }

        // Reverse so min at end for pop()
        q.sort_by(|a, b| a.distance.cmp(&b.distance).reverse());
    }

    None
}
