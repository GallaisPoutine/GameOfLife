
pub struct Cell {
    alive : bool,
    x : u32,
    y : u32,
}

impl Cell {
    pub fn new(alive : bool, x : u32, y : u32) -> Cell {
        Cell {
            alive,
            x,
            y,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn check_neighbours(&self) {

    }
}