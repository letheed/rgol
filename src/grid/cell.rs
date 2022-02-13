/// A cell that can be dead or alive.

// # Invariants
//
// When creating a new `Cell`, both flags must be set to the same value.
// The fields must remain equal, excepted during the tick, when a living cell may
// die and a dead cell may come alive.
#[allow(clippy::manual_non_exhaustive)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    /// Is the cell alive?
    pub alive: bool,
    /// Will the cell live or die?
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
