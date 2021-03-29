use std::fmt::{self, Display};

use map::Map;

mod map;

/// World for the Game of Life.
///
/// Contains a `Map` of `Cell`s and keeps track of the generation.
pub struct World {
    map: Map,
    generation: u64,
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{:?}, generation: {}", self.map, self.map.dim(), self.generation)
    }
}

impl World {
    /// Reads a file as a map and seeds a world with it.
    pub fn load(filename: &str) -> anyhow::Result<Self> {
        let map = std::fs::read_to_string(filename)?.parse()?;
        Ok(Self::new(map))
    }

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
    fn new(seed: Map) -> Self {
        Self { map: seed, generation: 0 }
    }
}
