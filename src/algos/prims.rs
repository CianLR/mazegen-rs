use rand::prelude::*;

use crate::algos::algo::MazeAlgo;
use crate::maze::Maze;

pub struct PrimsAlgo {
    animate: bool,
}

impl PrimsAlgo {
    pub fn new(animate: bool) -> PrimsAlgo {
        PrimsAlgo { animate: animate }
    }

    fn prims(&self, maze: &mut Maze) -> Result<(), String> {
        let size = maze.get_size();
        let mut rng = rand::thread_rng();
        let mut in_maze = vec![vec![false; size]; size];
        let mut walls: Vec<_> = maze.get_adjacent(0, 0)
            .into_iter()
            .map(|(x, y)| (0, 0, x, y))
            .collect();
        in_maze[0][0] = true;
        while !walls.is_empty() {
            let (x, y, x2, y2) = walls.remove(rng.gen_range(0, walls.len()));
            if in_maze[x2][y2] {
                continue;  // Already connected.
            }
            in_maze[x2][y2] = true;
            if self.animate {
                self.frame(maze, 20);
            }
            maze.remove_wall(x, y, x2, y2);
            walls.extend(maze.get_adjacent(x2, y2)
                .into_iter()
                .filter(|(x3, y3)| !in_maze[*x3][*y3])
                .map(|(x3, y3)| (x2, y2, x3, y3)));
        }
        Ok(())
    }
}

impl MazeAlgo for PrimsAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String> {
        self.prims(maze)
    }
}
