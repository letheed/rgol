use cell::Cell;
use std::{
    error::Error,
    fmt::{self, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

mod cell;

#[derive(Debug)]
pub(super) enum ParseMapError {
    Empty,
    NotRectangular,
}

impl Error for ParseMapError {}

impl Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMapError::Empty => write!(f, "map is empty"),
            ParseMapError::NotRectangular => write!(f, "map is not rectangular"),
        }
    }
}

/// A rectangular map of the world containing cells.
pub(super) struct Map {
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
        use ParseMapError::*;

        if s.is_empty() {
            return Err(Empty);
        }
        let lines = s.lines().collect::<Vec<_>>();
        if lines.iter().all(|line| line.is_empty()) {
            return Err(Empty);
        }
        let (nrow, ncol) = (lines.len(), lines[0].chars().filter(|c| !c.is_whitespace()).count());
        let mut cells = Vec::with_capacity(nrow * ncol);
        for line in lines {
            let mut nchar = 0;
            for c in line.chars().filter(|c| !c.is_whitespace()) {
                nchar += 1;
                cells.push(match c {
                    '·' => Cell::new_dead(),
                    _ => Cell::new_alive(),
                });
            }
            if nchar != ncol {
                return Err(NotRectangular);
            }
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
    pub(super) fn dim(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Increments the time by one tick.
    ///
    /// Births and deaths happen simultaneously according to the rules
    /// of Conway's Game of Life, after which the map contains
    /// the next generation.
    pub(super) fn next(&mut self) {
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let live_neighbours = self.live_neighbours((i, j));
                let cell = &mut self[(i, j)];
                if !cell.alive {
                    if live_neighbours == 3 {
                        cell.lives = true;
                    }
                } else if live_neighbours < 2 || 3 < live_neighbours {
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
    /// Panics if the number of cells in not `nrow * ncol`.
    fn from_parts(cells: Vec<Cell>, (nrow, ncol): (usize, usize)) -> Self {
        assert_eq!(cells.len(), nrow * ncol);
        Self { cells: cells.into_boxed_slice(), nrow, ncol }
    }

    /// Returns the number of living neighbours for a cell.
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
