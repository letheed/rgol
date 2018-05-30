use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use self::cell::Cell;
use self::error::ParseMapError;

mod cell;
mod error;

pub struct Map {
    data: Box<[Cell]>,
    nrow: usize,
    ncol: usize,
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::error::ParseMapError::*;

        if s.is_empty() {
            return Err(Empty);
        }
        let lines = s.lines().collect::<Vec<_>>();
        let (nrow, ncol) = (
            lines.len(),
            lines[0].chars().filter(|c| !c.is_whitespace()).count(),
        );
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
        Ok(Map::from_parts_unchecked(cells, (nrow, ncol)))
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Cell;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i * self.ncol + j]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[i * self.ncol + j]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut map_str = String::with_capacity(self.nrow * (self.ncol * 3 + 1));
        for row in self.data.chunks(self.ncol) {
            for cell in row {
                if cell.alive {
                    map_str.push_str(" X");
                } else {
                    map_str.push_str(" ·");
                }
            }
            map_str.push('\n');
        }
        f.write_str(&map_str)
    }
}

impl Map {
    pub fn dim(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    pub fn next(&mut self) {
        use std::cmp::min;

        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let mut live_neighbours = 0u8;
                for m in i.saturating_sub(1)..min(i + 2, self.nrow) {
                    for n in j.saturating_sub(1)..min(j + 2, self.ncol) {
                        if self[(m, n)].alive && (m != i || n != j) {
                            live_neighbours += 1;
                        }
                    }
                }
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
        for cell in &mut *self.data {
            cell.alive = cell.lives;
        }
    }
}

impl Map {
    fn from_parts_unchecked(cells: Vec<Cell>, (nrow, ncol): (usize, usize)) -> Self {
        assert_eq!(cells.len(), nrow * ncol);
        Map {
            data: cells.into_boxed_slice(),
            nrow,
            ncol,
        }
    }
}
