use std::boxed::Box;

use crate::maze::Maze;
use crate::algos::*;

pub trait MazeAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String>;
}

pub trait MazeAlgo2 {
    fn generate(maze: &mut Maze) -> Result<(), String>;
}

pub fn get_algorithm(algo: &String) -> Result<Box<MazeAlgo>, String> {
    match algo.as_ref() {
        "dfs" => Ok(Box::new(dfs::DfsAlgo::new())),
        "kruskals" => Ok(Box::new(kruskals::KruskalsAlgo::new())),
        "wilsons" => Ok(Box::new(wilsons::WilsonsAlgo::new())),
        "ellers" => Ok(Box::new(ellers::EllersAlgo::new())),
        "prims" => Ok(Box::new(prims::PrimsAlgo::new())),
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

