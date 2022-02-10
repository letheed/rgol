//! A simple implementation of Conway's Game of Life.
//!
//! This library is not meant for general usage.
//! It does not use any of the established file formats for the Game of Life.
//!
//! It can only do three things: parse a map to create a world, make it tick,
//! and print it.
//!
//! # Maps
//!
//! Maps must be rectangular. Whitespace is ignored.
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

use map::Map;
pub use map::ParseMapError;

mod map;

/// World for the Game of Life.
///
/// Contains a `Map` of `Cell`s and keeps track of the generation.
pub struct World {
    map: Map,
    generation: u64,
}

impl FromStr for World {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{:?}, generation: {}", self.map, self.map.dim(), self.generation)
    }
}

impl World {
    /// Increments the time by one tick.
    ///
    /// The next generation will replace the current one.
    pub fn next(&mut self) {
        self.generation += 1;
        self.map.next();
    }
}

impl World {
    /// Creates a new `World` from a `Map` seed (ie. generation 0).
    const fn new(seed: Map) -> Self {
        Self { map: seed, generation: 0 }
    }
}
