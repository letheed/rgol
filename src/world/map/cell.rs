pub struct Cell {
    pub alive: bool,
    pub lives: bool,
}

impl Cell {
    pub fn new_alive() -> Self {
        Cell { alive: true, lives: true }
    }

    pub fn new_dead() -> Self {
        Cell { alive: false, lives: false }
    }
}
