use rand::prelude::*;

use crate::algos::algo::MazeAlgo;
use crate::maze::Maze;
use crate::algos::util;

pub struct KruskalsAlgo {
    animate: bool
}

impl KruskalsAlgo {
    pub fn new(animate: bool) -> KruskalsAlgo {
        KruskalsAlgo { animate: animate }
    }

    fn kruskals(&self, maze: &mut Maze) -> Result<(), String> {
        let mut uf = util::UnionFind::new();
        let mut walls = vec![];
        let size = maze.get_size();
        for x in 0..size {
            for y in 0..size {
                for (x2, y2) in maze.get_adjacent(x, y) {
                    walls.push((x, y, x2, y2));
                }
            }
        }
        walls.shuffle(&mut rand::thread_rng());
        while !walls.is_empty() {
            let (x, y, x2, y2) = walls.pop().unwrap();
            if uf.connected(&(x, y), &(x2, y2)) {
                continue;
            }
            uf.join(&(x, y), &(x2, y2));
            if self.animate {
                self.frame(maze, 20);
            }
            maze.remove_wall(x, y, x2, y2);
        }
        Ok(())
    }
}

impl MazeAlgo for KruskalsAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String> {
        self.kruskals(maze)
    }
}
