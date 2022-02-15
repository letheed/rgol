use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use cell::Cell;
use error::{GridSizeError, ParseGridError};

pub mod cell;
pub mod error;

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

impl AsRef<[Cell]> for Grid {
    fn as_ref(&self) -> &[Cell] {
        &self.cells
    }
}

impl AsMut<[Cell]> for Grid {
    fn as_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
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
    /// Creates a new `Grid` of dead cells.
    ///
    /// # Errors
    ///
    /// Returns an error if `nrow` or `ncol` is 0.
    pub fn new((nrow, ncol): (usize, usize)) -> Result<Self, GridSizeError> {
        if nrow == 0 || ncol == 0 {
            Err(GridSizeError::Zero)
        } else {
            Ok(Self::from_parts(vec![Cell::new_dead(); nrow * ncol], (nrow, ncol)))
        }
    }

    /// Returns the number of rows and columns of the grid.
    #[must_use]
    pub const fn size(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Returns the number of living neighbors for a cell.
    #[must_use]
    pub fn live_neighbors(&self, (i, j): (usize, usize)) -> u8 {
        use std::cmp::min;

        let mut live_neighbors = 0;
        for m in i.saturating_sub(1)..min(i + 2, self.nrow) {
            for n in j.saturating_sub(1)..min(j + 2, self.ncol) {
                if self[(m, n)].alive && (m != i || n != j) {
                    live_neighbors += 1;
                }
            }
        }
        live_neighbors
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
        assert!(nrow != 0, "attempted to create a zero-sized grid");
        assert!(ncol != 0, "attempted to create a zero-sized grid");
        assert_eq!(nrow * ncol, cells.len(), "putative grid size does not match the number of cells");
        Self { cells: cells.into_boxed_slice(), nrow, ncol }
    }
}
