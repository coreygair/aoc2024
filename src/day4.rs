use crate::util::grid::Grid;

pub type Input = Grid<char>;

pub fn parse(input: &str) -> Input {
    Grid::from(input)
}

pub fn part1(input: &Input) -> u32 {
    let mut count = 0;

    for row in 0..input.n_rows() {
        for col in 0..input.n_cols() {
            if input.is_row_col(row, col, 'X') {
                for dy in -1..1 {
                    'dx: for dx in -1..1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        for (step, c) in [(1, 'M'), (2, 'A'), (3, 'S')] {
                            if !input.is_row_col(row + (dy * step), col + (dx * step), c) {
                                continue 'dx;
                            }
                        }

                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn part2(input: &Input) -> u32 {
    let mut count = 0;

    for row in 0..input.n_rows() {
        for col in 0..input.n_cols() {
            if input.is_row_col(row, col, 'A') {
                if ((input.is_row_col(row - 1, col - 1, 'S')
                    && input.is_row_col(row + 1, col + 1, 'M'))
                    || (input.is_row_col(row - 1, col - 1, 'M')
                        && input.is_row_col(row + 1, col + 1, 'S')))
                    && ((input.is_row_col(row + 1, col - 1, 'S')
                        && input.is_row_col(row - 1, col + 1, 'M'))
                        || (input.is_row_col(row + 1, col - 1, 'M')
                            && input.is_row_col(row - 1, col + 1, 'S')))
                {
                    count += 1;
                }
            }
        }
    }

    count
}
