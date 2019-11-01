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
    const GRID_SIZE : usize = height * width;

    let grid : [cell::Cell; GRID_SIZE];

    for i in 0..height {
        for j in 0..width {
		    // print!("{}{}", termion::clear::All, termion::cursor::Goto(i, j));
            print!("o");
            let grid[i][j] = Cell::new(alive, i, j);
        }
    }
}
