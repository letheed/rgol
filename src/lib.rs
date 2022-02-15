//! A simple implementation of Conway's Game of Life for terminal.
//!
//! # About
//!
//! This library is not meant for general usage.
//!
//! It does not use any of the established file formats for the Game of Life,
//! nor does it make any particular attempt at speed.
//!
//! It can parse a [`String`] to create a [`Grid`], make it tick, and print it.
//!
//! [`World`] will do the same and keep track of the generation as well.
//!
//! # Grid format
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

pub use grid::{
    cell::Cell,
    error::{GridSizeError, ParseGridError},
    Grid,
};

mod grid;

/// A convenience wrapper for [`Grid`]. It keeps track of the generation.
///
/// It will print the size of the grid and the generation as well.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct World {
    /// The grid of cells.
    grid: Grid,
    /// Number of generations passed.
    ///
    /// Starts at zero (the seed) and increases by one for every call to
    /// [`tick`](Self::tick).
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
        write!(f, "{}\n{:?}, generation: {}", self.grid, self.grid.size(), self.generation)
    }
}

impl World {
    /// Make time tick. The next generation of cells will replace
    /// the current one.
    ///
    /// Births and deaths happen simultaneously according to the rules
    /// of Conway's Game of Life, after which the grid contains
    /// the next generation.
    pub fn tick(&mut self) {
        self.generation += 1;
        self.grid.tick();
    }
}

impl World {
    /// Creates a new `World` from a seed (generation 0).
    #[must_use]
    const fn new(seed: Grid) -> Self {
        Self { grid: seed, generation: 0 }
    }
}
