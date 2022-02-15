use std::{
    error::Error,
    fmt::{self, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use cell::Cell;

pub mod cell;

/// An error returned when parsing a [`Grid`] from a string failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseGridError {
    /// The grid is empty.
    Empty,
    /// The grid is not rectangular.
    NotRectangular {
        /// Line on which the error occurred.
        line: usize,
        /// Number of cells found.
        found: usize,
        /// Number of cells expected.
        expected: usize,
    },
}

impl Error for ParseGridError {}

impl Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseGridError::Empty => "grid is empty".fmt(f),
            ParseGridError::NotRectangular { line, found, expected } => {
                write!(f, "grid is not rectangular (line {line}, expected {expected} cells, found {found})")
            }
        }
    }
}

/// An opaque container representing a rectangular grid of [`Cell`]s for
/// playing the Game of Life.
///
/// The grid is stored in row-major order.
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    /// Array storing the cells in row-major order.
    cells: Box<[Cell]>,
    /// Number of rows in the grid.
    nrow: usize,
    /// Number of columns in the grid.
    ncol: usize,
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseGridError::{Empty, NotRectangular};

        let mut nrow = 0;
        let mut ncol = 0;
        let mut cells = Vec::with_capacity(s.len());
        for line in s.lines() {
            nrow += 1;
            let mut ncell = 0;
            for c in line.chars().filter(|c| !c.is_whitespace()) {
                ncell += 1;
                cells.push(match c {
                    '·' => Cell::new_dead(),
                    _ => Cell::new_alive(),
                });
            }
            if nrow == 1 {
                ncol = ncell;
            } else if ncell != ncol {
                return Err(NotRectangular { line: nrow, found: ncell, expected: ncol });
            }
        }
        if cells.is_empty() {
            return Err(Empty);
        }
        Ok(Self::from_parts(cells, (nrow, ncol)))
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.cells[i * self.ncol + j]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[i * self.ncol + j]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.chunks(self.ncol) {
            for cell in row {
                if cell.alive {
                    write!(f, " X")?;
                } else {
                    write!(f, " ·")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    /// Returns the number of rows and columns of the grid.
    #[must_use]
    pub const fn dim(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Make time tick. The next generation of cells will replace
    /// the current one.
    ///
    /// Births and deaths happen simultaneously according to the rules
    /// of Conway's Game of Life, after which the grid contains
    /// the next generation.
    pub fn tick(&mut self) {
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let live_neighbors = self.live_neighbors((i, j));
                let cell = &mut self[(i, j)];
                if !cell.alive {
                    if live_neighbors == 3 {
                        cell.lives = true;
                    }
                } else if live_neighbors != 2 && live_neighbors != 3 {
                    cell.lives = false;
                }
            }
        }
        for cell in &mut *self.cells {
            cell.alive = cell.lives;
        }
    }
}

impl Grid {
    /// Creates a `Grid` from a vector of `Cell`s and a matching `Grid` size.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * nrow or ncol is 0.
    /// * the number of cells is not `nrow * ncol`.
    fn from_parts(cells: Vec<Cell>, (nrow, ncol): (usize, usize)) -> Self {
        assert_eq!(cells.len(), nrow * ncol);
        Self { cells: cells.into_boxed_slice(), nrow, ncol }
    }

    /// Returns the number of living neighbors for a cell.
    #[must_use]
    fn live_neighbors(&self, (i, j): (usize, usize)) -> u8 {
        use std::cmp::min;

        let mut live_neighbors = 0_u8;
        for m in i.saturating_sub(1)..min(i + 2, self.nrow) {
            for n in j.saturating_sub(1)..min(j + 2, self.ncol) {
                if self[(m, n)].alive && (m != i || n != j) {
                    live_neighbors += 1;
                }
            }
        }
        live_neighbors
    }
}
