#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }

    pub fn moved_in(self, direction: Direction) -> Self {
        let (d_row, d_col) = direction.to_row_col_diff();
        Position {
            row: self.row + d_row,
            col: self.col + d_col,
        }
    }
}

macro_rules! impl_position_ops {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = Position;

            fn add(self, rhs: $rhs) -> Self::Output {
                Position::new(self.row + rhs.row, self.col + rhs.col)
            }
        }
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Position;

            fn sub(self, rhs: $rhs) -> Self::Output {
                Position::new(self.row - rhs.row, self.col - rhs.col)
            }
        }
    };
}
impl_position_ops!(Position, Position);
impl_position_ops!(Position, &Position);
impl_position_ops!(&Position, Position);
impl_position_ops!(&Position, &Position);

macro_rules! impl_position_assign_ops {
    ($rhs:ty) => {
        impl std::ops::AddAssign<$rhs> for Position {
            fn add_assign(&mut self, rhs: $rhs) {
                (*self).row += rhs.row;
                (*self).col += rhs.col;
            }
        }
        impl std::ops::SubAssign<$rhs> for Position {
            fn sub_assign(&mut self, rhs: $rhs) {
                (*self).row += rhs.row;
                (*self).col += rhs.col;
            }
        }
    };
}
impl_position_assign_ops!(Position);
impl_position_assign_ops!(&Position);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_arrow(c: char) -> Option<Self> {
        Some(match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => return None,
        })
    }

    fn to_row_col_diff(self) -> (i32, i32) {
        match self {
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
        }
    }

    // 90 deg rotation.
    pub fn rotated_clockwise(self) -> Self {
        match self {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    pub fn reversed(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
        }
    }

    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}
