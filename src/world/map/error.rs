use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use self::ParseMapError::*;

#[derive(Debug)]
pub enum ParseMapError {
    Empty,
    NotRectangular,
}

impl Display for ParseMapError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl Error for ParseMapError {
    fn description(&self) -> &str {
        match *self {
            Empty          => "map is empty",
            NotRectangular => "map is not rectangular",
        }
    }
}
