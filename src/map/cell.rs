/// A cell that can be dead or alive.
///
/// When creating a new cell, both flags should be set to the same value.
pub(crate) struct Cell {
    /// Is the cell alive?
    pub(super) alive: bool,
    /// Does the cell live or die at the end of this turn?
    pub(super) lives: bool,
}

impl Cell {
    /// Returns a new living cell.
    pub(super) fn new_alive() -> Self {
        Self { alive: true, lives: true }
    }

    /// Returns a new dead cell.
    pub(super) fn new_dead() -> Self {
        Self { alive: false, lives: false }
    }
}
