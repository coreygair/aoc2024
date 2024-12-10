use std::marker::PhantomData;

use super::position::Position;

// AoC always has quite a few days where the input is a grid of characters,
// might as well boilerplate it out early.
#[derive(Clone)]
pub struct Grid<E>(Vec<Vec<E>>);

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

impl<E: From<char> + std::fmt::Debug> std::fmt::Debug for Grid<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter() {
            write!(f, "{:?}\n", l)?;
        }
        Ok(())
    }
}

impl<E> Grid<E> {
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

impl<E: Clone> Grid<E> {
    pub fn iter(&self) -> GridIterator<'_, E> {
        GridIterator {
            grid: self,
            position: Position::new(0, 0),
        }
    }
}

impl<E: PartialEq> Grid<E> {
    pub fn is(&self, pos: Position, element: E) -> bool {
        self.get(pos).map(|e| *e == element).unwrap_or(false)
    }

    pub fn is_row_col(&self, row: i32, col: i32, element: E) -> bool {
        self.is(Position::new(row, col), element)
    }
}

pub struct GridIterator<'a, E> {
    grid: &'a Grid<E>,
    position: Position,
}

impl<E: Clone> Iterator for GridIterator<'_, E> {
    type Item = (Position, E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.row >= self.grid.n_rows() {
            return None;
        }

        // SAFETY: Just did bounds checking.
        let item = Some((self.position, self.grid.get(self.position).unwrap().clone()));

        self.position.col = (self.position.col + 1) % self.grid.n_cols();
        if self.position.col == 0 {
            self.position.row += 1;
        }

        item
    }
}
