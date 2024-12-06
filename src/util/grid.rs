// AoC always has quite a few days where the input is a grid of characters,
// might as well boilerplate it out early.
#[derive(Clone, Debug)]
pub struct Grid<E: From<char>>(Vec<Vec<E>>);

impl<E: From<char>> From<&str> for Grid<E> {
    fn from(value: &str) -> Self {
        Grid(
            value
                .lines()
                .map(|l| l.chars().map(E::from).collect())
                .collect(),
        )
    }
}

impl<E: From<char>> Grid<E> {
    // Doing most ops with `i32` for easy add/sub ops with bounds checks in accessor methods.
    // Size of AoC inputs is not going to be a concern.
    pub fn n_rows(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn n_cols(&self) -> i32 {
        self.0[0].len() as i32
    }

    pub fn get(&self, pos: Position) -> Option<&E> {
        let row: usize = pos.row.try_into().ok()?;
        let col: usize = pos.col.try_into().ok()?;
        self.0.get(row)?.get(col)
    }

    pub fn set(&mut self, pos: Position, element: E) {
        let row: usize = match pos.row.try_into() {
            Ok(x) => x,
            Err(_) => return,
        };
        let col: usize = match pos.col.try_into() {
            Ok(x) => x,
            Err(_) => return,
        };
        if let Some(row) = self.0.get_mut(row) {
            if let Some(e) = row.get_mut(col) {
                *e = element
            }
        }
    }
}

impl<E: From<char> + PartialEq> Grid<E> {
    pub fn is(&self, pos: Position, element: E) -> bool {
        self.get(pos).map(|e| *e == element).unwrap_or(false)
    }

    pub fn is_row_col(&self, row: i32, col: i32, element: E) -> bool {
        self.is(Position::new(row, col), element)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    row: i32,
    col: i32,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_row_col_diff(self) -> (i32, i32) {
        match self {
            Direction::Down => (1, 0),
            Direction::Right => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Left => (0, 1),
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
}
