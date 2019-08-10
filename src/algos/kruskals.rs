use rand::prelude::*;

use crate::algos::algo;
use crate::maze::Maze;
use crate::algos::util;

pub struct KruskalsAlgo;

impl KruskalsAlgo {
    fn kruskals(maze: &mut Maze) -> Result<(), String> {
        let mut uf = util::UnionFind::new();
        let mut walls = vec![];
        let size = maze.get_size();
        for x in 0..size {
            for y in 0..size {
                for (x2, y2, d) in maze.get_adjacent(x, y) {
                    walls.push((x, y, x2, y2, d));
                }
            }
        }
        walls.shuffle(&mut rand::thread_rng());
        while !walls.is_empty() {
            let (x, y, x2, y2, d) = walls.pop().unwrap();
            if uf.connected(&(x, y), &(x2, y2)) {
                continue;
            }
            uf.join(&(x, y), &(x2, y2));
            maze.rm_wall(x, y, d);
            maze.rm_wall(x2, y2, d.opposite());
        }
        Ok(())
    }
}

impl algo::MazeAlgo for KruskalsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        KruskalsAlgo::kruskals(maze)
    }
}
