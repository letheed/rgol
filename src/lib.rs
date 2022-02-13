//! A simple implementation of Conway's Game of Life.
//!
//! This library is not meant for general usage.
//! It does not use any of the established file formats for the Game of Life.
//!
//! It can only do three things: parse a string to create a [`Grid`], make it tick,
//! and print it.
//!
//! # Grids
//!
//! Grids must be rectangular. Whitespace is ignored.
//!
//! ‘·’ (U+00B7 MIDDLE DOT) is a dead cell. Anything else is a living cell.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![deny(unsafe_code)]

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use grid::Grid;
pub use grid::ParseGridError;

mod grid;

/// World for the Game of Life.
///
/// Contains a `Grid` of `Cell`s and keeps track of the generation.
pub struct World {
    grid: Grid,
    generation: u64,
}

impl FromStr for World {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{:?}, generation: {}", self.grid, self.grid.dim(), self.generation)
    }
}

impl World {
    /// Increments the time by one tick.
    ///
    /// The next generation will replace the current one.
    pub fn next(&mut self) {
        self.generation += 1;
        self.grid.next();
    }
}

impl World {
    /// Creates a new `World` from a `Grid` seed (ie. generation 0).
    #[must_use]
    const fn new(seed: Grid) -> Self {
        Self { grid: seed, generation: 0 }
    }
}
