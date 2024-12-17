use std::collections::{HashMap, HashSet};

use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Start,
    End,
    Wall,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Cell::Start,
            'E' => Cell::End,
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            _ => unreachable!(),
        }
    }
}

type Input = Grid<Cell>;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> u64 {
    let start = input.find(Cell::Start).unwrap();

    let mut visited = HashSet::new();

    let mut q = Vec::new();
    q.push(Entry {
        pos: start,
        dir: Direction::Right,
        score: 0,
    });

    while let Some(Entry { pos, dir, score }) = q.pop() {
        let ve = VisitedEntry { pos, dir };
        if visited.contains(&ve) {
            continue;
        }
        visited.insert(ve);

        if *input.get(pos).unwrap() == Cell::End {
            return score;
        }

        for new_dir in Direction::ALL {
            if new_dir == dir.reversed() {
                continue;
            }

            let new_pos = pos.moved_in(new_dir);
            if *input.get(new_pos).unwrap() == Cell::Wall {
                continue;
            }

            let new_score = score + if new_dir != dir { 1001 } else { 1 };

            q.push(Entry {
                pos: new_pos,
                dir: new_dir,
                score: new_score,
            });
        }

        // Reverse so min at end for pop()
        q.sort_by(|a, b| a.score.cmp(&b.score).reverse());
    }

    unreachable!("didn't find the end")
}

pub fn part2(input: &Input) -> u64 {
    let start = input.find(Cell::Start).unwrap();

    let mut visited = HashSet::new();

    let mut prevs = HashMap::new();
    prevs.insert(
        PreviousKey {
            pos: start,
            score: 0,
        },
        HashSet::from([start]),
    );

    let mut q = Vec::new();
    q.push(Entry {
        pos: start,
        dir: Direction::Right,
        score: 0,
    });

    while let Some(Entry { pos, dir, score }) = q.pop() {
        let ve = VisitedEntry { pos, dir };
        if visited.contains(&ve) {
            continue;
        }
        visited.insert(ve);

        if *input.get(pos).unwrap() == Cell::End {
            break;
        }

        for new_dir in Direction::ALL {
            if new_dir == dir.reversed() {
                continue;
            }

            let new_pos = pos.moved_in(new_dir);
            if *input.get(new_pos).unwrap() == Cell::Wall {
                continue;
            }

            let new_score = score + if new_dir != dir { 1001 } else { 1 };

            q.push(Entry {
                pos: new_pos,
                dir: new_dir,
                score: new_score,
            });

            // Using remove() here avoids needing to clone the `ps` allocation to avoid having a & and &mut to prevs at the same time
            // (without using Cell/RefCell/etc. which I cba)
            // Doesn't actually save much performance, oh well
            let ps = prevs.remove(&PreviousKey { pos, score }).unwrap();
            prevs
                .entry(PreviousKey {
                    pos: new_pos,
                    score: new_score,
                })
                .or_insert(HashSet::from([new_pos]))
                .extend(ps.iter());
            prevs.insert(PreviousKey { pos, score }, ps);
        }

        // Reverse so min at end for pop()
        q.sort_by(|a, b| a.score.cmp(&b.score).reverse());
    }

    let end_pos = input.find(Cell::End).unwrap();
    prevs
        .iter()
        .find(|(k, _)| k.pos == end_pos)
        .unwrap()
        .1
        .len() as u64
}

#[derive(PartialEq, Eq, Hash)]
struct VisitedEntry {
    pos: Position,
    dir: Direction,
}

struct Entry {
    pos: Position,
    dir: Direction,
    score: u64,
}

#[derive(PartialEq, Eq, Hash)]
struct PreviousKey {
    pos: Position,
    score: u64,
}
