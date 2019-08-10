use rand::prelude::*;

use crate::algos::algo;
use crate::maze::Maze;

pub struct PrimsAlgo;

impl PrimsAlgo {
    fn prims(maze: &mut Maze) -> Result<(), String> {
        let size = maze.get_size();
        let mut rng = rand::thread_rng();
        let mut in_maze = vec![vec![false; size]; size];
        let mut walls: Vec<_> = maze.get_adjacent(0, 0)
            .into_iter()
            .map(|(x, y, d)| (0, 0, x, y, d))
            .collect();
        in_maze[0][0] = true;
        while !walls.is_empty() {
            let (x, y, x2, y2, d) = walls.remove(rng.gen_range(0, walls.len()));
            if in_maze[x2][y2] {
                continue;  // Already connected.
            }
            in_maze[x2][y2] = true;
            maze.print();
            std::thread::sleep(std::time::Duration::from_millis(50));
            maze.rm_wall(x, y, d);
            maze.rm_wall(x2, y2, d.opposite());
            walls.extend(maze.get_adjacent(x2, y2)
                .into_iter()
                .filter(|(x3, y3, _)| !in_maze[*x3][*y3])
                .map(|(x3, y3, d2)| (x2, y2, x3, y3, d2)));
        }
        Ok(())
    }
}

impl algo::MazeAlgo for PrimsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        PrimsAlgo::prims(maze)
    }
}
