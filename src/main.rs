mod cell;

use std::env;
use termion;

const CELL : &str = "x";
const HOLE : &str = " ";

fn main() {
    println!("Hello, world!");
    // Get cli args
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let (height, width) = termion::terminal_size().unwrap();
    // let grid_size = 

    let mut grid = Vec::<cell::Cell>::with_capacity((height * width) as usize);

    for i in 0..height as u32 {
        for j in 0..width as u32 {
            grid.push(cell::Cell::new(true, i, j));
        }
    }
}
