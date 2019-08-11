use std::boxed::Box;
use std::thread::sleep;
use std::time::Duration;

use crate::maze::Maze;
use crate::algos::*;

pub trait MazeAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String>;

    fn frame(&self, maze: &Maze, millis: u64) {
        maze.print_and_reset();
        sleep(Duration::from_millis(millis));
    }

}

pub fn get_algorithm(algo: &String, ani: bool) -> Result<Box<MazeAlgo>, String> {
    match algo.as_ref() {
        "dfs" => Ok(Box::new(DfsAlgo::new(ani))),
        "kruskals" => Ok(Box::new(KruskalsAlgo::new(ani))),
        "wilsons" => Ok(Box::new(WilsonsAlgo::new(ani))),
        "ellers" => Ok(Box::new(EllersAlgo::new(ani))),
        "prims" => Ok(Box::new(PrimsAlgo::new(ani))),
        _ => Err("Algorithm not found".to_string()),
    }
}

pub static ALGORITHMS: [&str; 5] = [
    "dfs",
    "kruskals",
    "wilsons",
    "ellers",
    "prims",
];

