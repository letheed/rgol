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
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filename)?;
        let mut map = String::new();
        file.read_to_string(&mut map)?;
        map.parse().map(World::new).map_err(Into::into)
    }

    pub fn next(&mut self) {
        self.iterations += 1;
        self.map.next();
    }
}

impl World {
    fn new(map: Map) -> Self {
        World { map: map, iterations: 0 }
    }
}
