#[derive(Clone, Copy, Debug)]
pub struct Cell {
    alive : bool,
    x : u32,
    y : u32,
}

// impl Clone for Cell {
//     pub fn clone (&self) -> Cell {
//         *self
//     }
// }

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
    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn process(&mut self, neighbours : u16) {
        self.alive = self.process_neighbours(neighbours);
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