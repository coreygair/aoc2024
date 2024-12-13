use std::collections::{HashSet, VecDeque};

use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    Grid::from(input)
}

pub fn part1(input: &Input) -> u64 {
    Region::find_all(input)
        .iter()
        .map(|r| r.area() * r.perimiter())
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    Region::find_all(input)
        .iter()
        .map(|r| r.area() * r.corners())
        .sum()
}

struct Region {
    points: HashSet<Position>,
}

impl Region {
    fn find_all(grid: &Input) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut seen: HashSet<Position> = HashSet::new();

        for (point, _) in grid.iter() {
            if seen.contains(&point) {
                continue;
            }

            let points = flood_fill(grid, point);

            seen.extend(points.iter());
            regions.push(Region { points });
        }

        regions
    }

    fn area(&self) -> u64 {
        self.points.len() as u64
    }

    fn perimiter(&self) -> u64 {
        self.points
            .iter()
            .map(|p| {
                Direction::ALL
                    .into_iter()
                    .map(|d| {
                        if !self.points.contains(&p.moved_in(d)) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    // This is awful but already late and it works so :)
    fn corners(&self) -> u64 {
        self.points
            .iter()
            .map(|p| {
                let is_outside = |d: Direction| -> bool {
                    let p = p.moved_in(d);
                    if !self.points.contains(&p) {
                        true
                    } else {
                        false
                    }
                };
                let neighbour_is_outside: [bool; 4] = [
                    is_outside(Direction::Up),
                    is_outside(Direction::Left),
                    is_outside(Direction::Down),
                    is_outside(Direction::Right),
                ];

                let outside_count = neighbour_is_outside.iter().cloned().filter(|b| *b).count();

                let outer_corners = if outside_count == 0 || outside_count == 1 {
                    0
                } else if outside_count == 2 {
                    if (neighbour_is_outside[0] && neighbour_is_outside[2])
                        || (neighbour_is_outside[1] && neighbour_is_outside[3])
                    {
                        0
                    } else {
                        1
                    }
                } else if outside_count == 3 {
                    2
                } else {
                    4
                };

                let mut inner_corners = 0;
                if !neighbour_is_outside[0]
                    && !neighbour_is_outside[1]
                    && !self
                        .points
                        .contains(&p.moved_in(Direction::Up).moved_in(Direction::Left))
                {
                    inner_corners += 1;
                }
                if !neighbour_is_outside[1]
                    && !neighbour_is_outside[2]
                    && !self
                        .points
                        .contains(&p.moved_in(Direction::Left).moved_in(Direction::Down))
                {
                    inner_corners += 1;
                }
                if !neighbour_is_outside[2]
                    && !neighbour_is_outside[3]
                    && !self
                        .points
                        .contains(&p.moved_in(Direction::Down).moved_in(Direction::Right))
                {
                    inner_corners += 1;
                }
                if !neighbour_is_outside[3]
                    && !neighbour_is_outside[0]
                    && !self
                        .points
                        .contains(&p.moved_in(Direction::Right).moved_in(Direction::Up))
                {
                    inner_corners += 1;
                }

                outer_corners + inner_corners
            })
            .sum()
    }
}

fn flood_fill(grid: &Grid<char>, from: Position) -> HashSet<Position> {
    let target = grid.get(from).unwrap().clone();

    let mut points = HashSet::new();
    points.insert(from);
    let mut q = VecDeque::new();
    q.push_back(from);

    while let Some(p) = q.pop_front() {
        for d in Direction::ALL {
            let p = p.moved_in(d);

            if points.contains(&p) {
                continue;
            }

            let Some(c) = grid.get(p) else {
                continue;
            };

            if *c != target {
                continue;
            }

            points.insert(p);
            q.push_back(p);
        }
    }

    points
}
