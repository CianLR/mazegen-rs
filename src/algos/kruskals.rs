use rand::prelude::*;

use crate::algos::algo;
use crate::maze::{Maze, Walls};

pub struct KruskalsAlgo;

impl KruskalsAlgo {
    fn kruskals(maze: &mut Maze) -> Result<(), String> {
        
        Ok(())
    }
}

impl algo::MazeAlgo for KruskalsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        KruskalsAlgo::kruskals(maze)
    }
}
