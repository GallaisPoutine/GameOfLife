#[derive(Clone, Copy, Debug)]
pub struct Cell {
    alive : bool,
    future : bool,
}

impl Cell {
    pub fn new(alive : bool, future : bool) -> Cell {
        Cell {
            alive,
            future
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_future(&mut self) {
        self.alive = self.future;
        self.future = false;
    }

    pub fn process(&mut self, neighbours : u16) {
        self.future = self.process_neighbours(neighbours);
    }

    // Process if alive according to neighbours
    pub fn process_neighbours(&self, neighbours : u16) -> bool {
        if self.alive && neighbours < 2 {
            // Underpopulation
            false
        } else if self.alive && (neighbours == 2 || neighbours == 3) {
            // Sustainment 
            true
        } else if self.alive && neighbours > 3 {
            // Overpopulation
            false
        } else if !self.alive && neighbours == 3 {
            // Reproduction
            true
        } else {
            // Other cases
            false
        }
    }
}