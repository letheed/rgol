#![allow(clippy::module_name_repetitions)]

use std::{
    error::Error,
    fmt::{self, Display},
};

/// An error returned when the size provided for the [`Grid`](super::Grid)
/// is wrong.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridSizeError {
    /// The size provided for the grid is 0.
    Zero,
}
impl Error for GridSizeError {}

impl Display for GridSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridSizeError::Zero => "size provided for the grid is 0".fmt(f),
        }
    }
}

/// An error returned when parsing a [`Grid`](super::Grid) from a [`String`]
/// failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseGridError {
    /// The grid is empty.
    Empty,
    /// The grid is not rectangular.
    NotRectangular {
        /// Line on which the error occurred.
        line: usize,
        /// Number of cells found.
        found: usize,
        /// Number of cells expected.
        expected: usize,
    },
}

impl Error for ParseGridError {}

impl Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseGridError::Empty => "grid is empty".fmt(f),
            ParseGridError::NotRectangular { line, found, expected } => {
                write!(f, "grid is not rectangular (line {line}, expected {expected} cells, found {found})")
            }
        }
    }
}
