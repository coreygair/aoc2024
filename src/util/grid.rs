// AoC always has quite a few days where the input is a grid of characters,
// might as well boilerplate it out early.
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

    pub fn get(&self, row: i32, col: i32) -> Option<&E> {
        let row: usize = row.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        self.0.get(row)?.get(col)
    }
}

impl<E: From<char> + PartialEq> Grid<E> {
    pub fn is(&self, row: i32, col: i32, element: E) -> bool {
        self.get(row, col).map(|e| *e == element).unwrap_or(false)
    }
}
