
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

    // Getter for cell ?

    // Get all neighbours and then process
    pub fn process_neighbours(&mut self) -> bool{
        for i in self.x-1..self.x+1 {
            for j in self.y-1..self.y+1 {
                // self.
            }
        }

        self.alive = self.check_neighbours(0);
        self.is_alive()
    }

    // Process if alive according to neighbours
    fn check_neighbours(&self, neighbours : u16) -> bool {
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