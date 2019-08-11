use std::boxed::Box;

use crate::maze::Maze;
use crate::algos::*;

pub trait MazeAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String>;
}

pub fn get_algorithm(algo: &String, ani: bool) -> Result<Box<MazeAlgo>, String> {
    match algo.as_ref() {
        "dfs" => Ok(Box::new(DfsAlgo::new(ani))),
        "kruskals" => Ok(Box::new(KruskalsAlgo::new(ani))),
        "wilsons" => Ok(Box::new(WilsonsAlgo::new())),
        "ellers" => Ok(Box::new(EllersAlgo::new())),
        "prims" => Ok(Box::new(PrimsAlgo::new())),
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

