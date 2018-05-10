use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use self::map::Map;

mod map;

pub struct World {
    map: Map,
    iterations: u64,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}\n{:?}, iterations: {}", self.map, self.map.dim(), self.iterations)
    }
}

impl World {
    pub fn load(filename: &str) -> Result<Self, Box<Error>> {
        let map = ::std::fs::read_to_string(filename)?.parse()?;
        Ok(World::new(map))
    }

    pub fn next(&mut self) {
        self.iterations += 1;
        self.map.next();
    }
}

impl World {
    fn new(map: Map) -> Self {
        World { map, iterations: 0 }
    }
}
