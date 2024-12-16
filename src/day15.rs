use crate::util::{
    grid::Grid,
    position::{Direction, Position},
};

type Input = (Grid<P1Cell>, Grid<P2Cell>, Vec<Direction>);

pub fn parse(input: &str) -> Input {
    let map = input
        .lines()
        .take_while(|l| *l != "")
        .fold(String::with_capacity(input.len()), |s, l| {
            format!("{}{}\n", s, l)
        })
        .as_str()
        .into();
    let map2 = input
        .lines()
        .take_while(|l| *l != "")
        .map(|l| {
            l.chars()
                .fold(String::with_capacity(l.len() * 2), |mut s, c| {
                    s.push_str(match c {
                        'O' => "[]",
                        '@' => "@.",
                        '.' => "..",
                        '#' => "##",
                        _ => unreachable!(),
                    });
                    s
                })
        })
        .fold(String::with_capacity(input.len() * 2), |s, l| {
            format!("{}{}\n", s, l)
        })
        .as_str()
        .into();
    let moves = input
        .lines()
        .skip_while(|l| *l != "")
        .flat_map(|l| l.chars().map(|c| Direction::from_arrow(c).unwrap()))
        .collect();

    (map, map2, moves)
}

pub fn part1((map, _, moves): &Input) -> u32 {
    let mut map = map.clone();

    let mut pos = map.find(P1Cell::Robot).unwrap();
    map.set(pos, P1Cell::Empty);

    for d in moves.iter().cloned() {
        let new_pos = pos.moved_in(d);

        match map.get(new_pos).unwrap() {
            P1Cell::Empty => pos = new_pos,
            P1Cell::Box => {
                let mut next_empty_pos = pos.moved_in(d);
                loop {
                    match map.get(next_empty_pos).unwrap() {
                        P1Cell::Empty => {
                            map.set(new_pos, P1Cell::Empty);
                            map.set(next_empty_pos, P1Cell::Box);
                            pos = new_pos;
                            break;
                        }
                        P1Cell::Box => {
                            next_empty_pos = next_empty_pos.moved_in(d);
                        }
                        P1Cell::Wall => {
                            break;
                        }
                        P1Cell::Robot => unreachable!(),
                    }
                }
            }
            P1Cell::Wall => {}
            P1Cell::Robot => unreachable!(),
        }
    }

    map.iter()
        .map(|(p, c)| {
            if matches!(c, P1Cell::Box) {
                100 * p.row as u32 + p.col as u32
            } else {
                0
            }
        })
        .sum::<u32>()
}

#[derive(Debug, Clone, PartialEq)]
pub enum P1Cell {
    Wall,
    Box,
    Robot,
    Empty,
}

impl From<char> for P1Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => P1Cell::Wall,
            'O' => P1Cell::Box,
            '@' => P1Cell::Robot,
            '.' => P1Cell::Empty,
            _ => unreachable!(),
        }
    }
}

pub fn part2((_, map, moves): &Input) -> u32 {
    let mut map = map.clone();

    let mut pos = map.find(P2Cell::Robot).unwrap();
    map.set(pos, P2Cell::Empty);

    for d in moves.iter().cloned() {
        let new_pos = pos.moved_in(d);

        match map.get(new_pos).unwrap() {
            P2Cell::Empty => pos = new_pos,
            P2Cell::LBox | P2Cell::RBox => {
                if matches!(d, Direction::Left | Direction::Right) {
                    if try_push_horizontal(&mut map, new_pos, d) {
                        pos = new_pos;
                    }
                } else {
                    if test_push_vertical(&map, new_pos, d) {
                        push_vertical(&mut map, new_pos, d);
                        pos = new_pos;
                    }
                }
            }
            P2Cell::Wall => {}
            P2Cell::Robot => unreachable!(),
        }
    }

    map.iter()
        .map(|(p, c)| {
            if matches!(c, P2Cell::LBox) {
                100 * p.row as u32 + p.col as u32
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn try_push_horizontal(map: &mut Grid<P2Cell>, box_position: Position, d: Direction) -> bool {
    let mut next_free_pos = box_position.moved_in(d);
    loop {
        match map.get(next_free_pos).unwrap() {
            P2Cell::Wall => {
                return false;
            }
            P2Cell::LBox | P2Cell::RBox => {}
            P2Cell::Empty => {
                map.set(box_position, P2Cell::Empty);
                let mut next_swap_pos = box_position.moved_in(d);
                while next_swap_pos != next_free_pos {
                    map.set(
                        next_swap_pos,
                        match map.get(next_swap_pos).unwrap() {
                            P2Cell::LBox => P2Cell::RBox,
                            P2Cell::RBox => P2Cell::LBox,
                            _ => unreachable!(),
                        },
                    );

                    next_swap_pos = next_swap_pos.moved_in(d);
                }
                map.set(
                    next_swap_pos,
                    if d == Direction::Right {
                        P2Cell::RBox
                    } else {
                        P2Cell::LBox
                    },
                );
                return true;
            }
            P2Cell::Robot => unreachable!(),
        }
        next_free_pos = next_free_pos.moved_in(d);
    }
}

fn test_push_vertical(map: &Grid<P2Cell>, box_position: Position, d: Direction) -> bool {
    let (l_pos, r_pos) = match map.get(box_position).unwrap() {
        P2Cell::LBox => (box_position, box_position.moved_in(Direction::Right)),
        P2Cell::RBox => (box_position.moved_in(Direction::Left), box_position),
        _ => unreachable!(),
    };

    match map.get(l_pos.moved_in(d)).unwrap() {
        P2Cell::Wall => return false,
        P2Cell::Empty => {}
        _ => {
            if !test_push_vertical(map, l_pos.moved_in(d), d) {
                return false;
            }
        }
    }
    match map.get(r_pos.moved_in(d)).unwrap() {
        P2Cell::Wall => return false,
        P2Cell::Empty => {}
        _ => {
            if !test_push_vertical(map, r_pos.moved_in(d), d) {
                return false;
            }
        }
    }

    true
}

/// Assumes a test_push_vertical on the same box_position returned true
fn push_vertical(map: &mut Grid<P2Cell>, box_position: Position, d: Direction) {
    let (l_pos, r_pos) = match map.get(box_position).unwrap() {
        P2Cell::LBox => (box_position, box_position.moved_in(Direction::Right)),
        P2Cell::RBox => (box_position.moved_in(Direction::Left), box_position),
        _ => unreachable!(),
    };

    match map.get(l_pos.moved_in(d)).unwrap() {
        P2Cell::LBox | P2Cell::RBox => {
            push_vertical(map, l_pos.moved_in(d), d);
        }
        P2Cell::Empty => {}
        _ => unreachable!(),
    }
    match map.get(r_pos.moved_in(d)).unwrap() {
        P2Cell::LBox | P2Cell::RBox => {
            push_vertical(map, r_pos.moved_in(d), d);
        }
        P2Cell::Empty => {}
        _ => unreachable!(),
    }

    map.set(l_pos, P2Cell::Empty);
    map.set(l_pos.moved_in(d), P2Cell::LBox);

    map.set(r_pos, P2Cell::Empty);
    map.set(r_pos.moved_in(d), P2Cell::RBox);
}

#[derive(Clone, PartialEq)]
pub enum P2Cell {
    Wall,
    LBox,
    RBox,
    Robot,
    Empty,
}

impl From<char> for P2Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => P2Cell::Wall,
            '[' => P2Cell::LBox,
            ']' => P2Cell::RBox,
            '@' => P2Cell::Robot,
            '.' => P2Cell::Empty,
            _ => unreachable!(),
        }
    }
}
