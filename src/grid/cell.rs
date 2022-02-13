/// A cell that can be dead or alive.
///
/// When creating a new cell, both flags should be set to the same value.
#[allow(clippy::manual_non_exhaustive)]
pub struct Cell {
    /// Is the cell alive?
    pub alive: bool,
    /// Does the cell live or die at the end of this turn?
    pub lives: bool,
    #[doc(hidden)]
    _private: (),
}

impl Cell {
    /// Returns a new living cell.
    #[must_use]
    pub const fn new_alive() -> Self {
        Self { alive: true, lives: true, _private: () }
    }

    /// Returns a new dead cell.
    #[must_use]
    pub const fn new_dead() -> Self {
        Self { alive: false, lives: false, _private: () }
    }
}
