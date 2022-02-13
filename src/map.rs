use std::{
    error::Error,
    fmt::{self, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use cell::Cell;

mod cell;

/// An error returned when parsing a Map from a string fails.
#[derive(Debug)]
pub enum ParseMapError {
    /// The map is empty.
    Empty,
    /// The map is not rectangular.
    NotRectangular {
        /// Line on which the error occured.
        line: usize,
        /// Number of cells found.
        found: usize,
        /// Number of cells expected.
        expected: usize,
    },
}

impl Error for ParseMapError {}

impl Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMapError::Empty => write!(f, "map is empty"),
            ParseMapError::NotRectangular { line, found, expected } => {
                write!(
                    f,
                    "map is not rectangular (line {}, expected {} cells, found {})",
                    line, expected, found
                )
            }
        }
    }
}

/// A rectangular map of the world containing cells.
pub struct Map {
    /// Array storing the cells in row-major order.
    cells: Box<[Cell]>,
    /// Number of rows in the map.
    nrow: usize,
    /// Number of columns in the map.
    ncol: usize,
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseMapError::{Empty, NotRectangular};

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

impl Index<(usize, usize)> for Map {
    type Output = Cell;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.cells[i * self.ncol + j]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[i * self.ncol + j]
    }
}

impl Display for Map {
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

impl Map {
    /// Returns the vertical and horizontal dimensions of the map.
    #[must_use]
    pub const fn dim(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Increments the time by one tick.
    ///
    /// Births and deaths happen simultaneously according to the rules
    /// of Conway's Game of Life, after which the map contains
    /// the next generation.
    pub fn next(&mut self) {
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let live_neighbours = self.live_neighbours((i, j));
                let cell = &mut self[(i, j)];
                if !cell.alive {
                    if live_neighbours == 3 {
                        cell.lives = true;
                    }
                } else if live_neighbours != 2 && live_neighbours != 3 {
                    cell.lives = false;
                }
            }
        }
        for cell in &mut *self.cells {
            cell.alive = cell.lives;
        }
    }
}

impl Map {
    /// Creates a `Map` from a vector of `Cell`s and a pair of dimensions.
    ///
    /// # Panics
    ///
    /// Panics if the number of cells is not `nrow * ncol`.
    #[must_use]
    fn from_parts(cells: Vec<Cell>, (nrow, ncol): (usize, usize)) -> Self {
        assert_eq!(cells.len(), nrow * ncol);
        Self { cells: cells.into_boxed_slice(), nrow, ncol }
    }

    /// Returns the number of living neighbours for a cell.
    #[must_use]
    fn live_neighbours(&self, (i, j): (usize, usize)) -> u8 {
        use std::cmp::min;

        let mut live_neighbours = 0_u8;
        for m in i.saturating_sub(1)..min(i + 2, self.nrow) {
            for n in j.saturating_sub(1)..min(j + 2, self.ncol) {
                if self[(m, n)].alive && (m != i || n != j) {
                    live_neighbours += 1;
                }
            }
        }
        live_neighbours
    }
}
