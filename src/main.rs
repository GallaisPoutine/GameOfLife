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
    // let mut grid = Vec::<Vec::<cell::Cell>::with_capacity(width as usize)>::with_capacity(height as usize);
    let mut grid = Vec::<Vec::<cell::Cell>>::new();
    let mut rng = rand::thread_rng();
    let timer = time::Duration::from_millis(100);
    
    // Grid creation
    for i in 0..height as i16 {
        let mut line = Vec::<cell::Cell>::new();
        for j in 0..width as i16 {
            line.push(cell::Cell::new(rng.gen::<bool>(), false));
        }
        grid.push(line);
    }

    // Debug
    // println!("{:?}", grid);
    assert_eq!(grid.len() as u16, height);
    assert_eq!(grid[0].len() as u16, width);

    println!("{:?}", grid.clone().iter().enumerate());
    
    loop {
        grid.iter().for_each(|line| 
                    line.iter().enumerate().for_each(|cell| {
                        cell.1.process(check_neighbours(&grid));
                        // if cell.is_alive() {
                        //     print!("{}", CELL);
                        // } else {
                        //     print!("{}", HOLE);
                        // }
                    })
                );

        print!("{}", termion::clear::All);
        thread::sleep(timer);
    }
}

fn check_neighbours(grid : &Vec<Vec<cell::Cell>>) -> u16 {
    let mut neighbours = 0;
    // for i in lhl-1 ..c.get_x()+1 {
    //     for j in c.get_y()-1 ..c.get_y()+1 {
    //         // Find cell corresponding to coord
    //         let cell = grid.iter().find(|c| c.get_x() == i && c.get_y() == j);//.unwrap();
    //         // println!("{:?}", cell);
    //         if cell.is_some() && cell.unwrap().is_alive() {
    //             neighbours += 1;
    //         }
    //     }
    // }
    neighbours
}

fn set_future_grid(grid : &Vec<Vec<cell::Cell>>) {

}
