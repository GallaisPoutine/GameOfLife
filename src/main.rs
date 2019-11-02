mod cell;

use std::{env, time, thread};
use rand::Rng;
use termion;

const CELL : &str = "x";
const HOLE : &str = " ";

fn main() {
    // Get cli args
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (height, width) = termion::terminal_size().unwrap();
    let mut grid = Vec::<cell::Cell>::with_capacity((height * width) as usize);
    let mut rng = rand::thread_rng();
    let timer = time::Duration::from_millis(250);
    
    // Grid creation
    for i in 0..height as u32 {
        for j in 0..width as u32 {
            grid.push(cell::Cell::new(rng.gen::<bool>(), i, j));
        }
    }

    // Debug
    // println!("{:?}", grid);
    assert_eq!(grid.len() as u16, height * width);

    loop {
        for mut cell in grid.clone() {
            cell.process(check_neighbours(grid.clone(), &cell));
            // if cell.process_neighbours() {
            if cell.is_alive() {
                print!("{}", CELL);
            } else {
                print!("{}", HOLE);
            }
        }
        print!("{}", termion::clear::All);
        thread::sleep(timer);
    }
}

fn check_neighbours(grid : Vec<cell::Cell>, c : &cell::Cell) -> u16 {
    let mut neighbours = 0;
    // TODO Resolve that
    // Panicked when does "-1" instruction
    for i in c.get_x()-1 ..c.get_x()+1 {
        for j in c.get_y()-1 ..c.get_y()+1 {
            // Find cell corresponding to coord
            // and do stuff
            let cell = grid.iter().find(|cell| cell.get_x() == i && cell.get_y() == j).unwrap();
            println!("{:?}", cell);
            if cell.is_alive() {
                neighbours += 1;
            }
        }
    }
    neighbours
}
