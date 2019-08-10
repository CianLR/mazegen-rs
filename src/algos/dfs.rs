use rand::prelude::*;

use crate::algos::algo;
use crate::maze::Maze;

pub struct DfsAlgo;

impl DfsAlgo {
    fn dfs(maze: &mut Maze, visited: &mut Vec<Vec<bool>>,
           x: usize, y: usize) -> Result<(), String> {
        let mut adj: Vec<(usize, usize)> = maze.get_adjacent(x, y)
            .into_iter()
            .filter(|&(nx, ny)| !visited[nx][ny])
            .collect();
        adj.shuffle(&mut rand::thread_rng());
        while !adj.is_empty() {
            let (nx, ny) = adj.pop().unwrap();
            if visited[nx][ny] {
                continue;
            }
            maze.remove_wall(x, y, nx, ny);
            visited[nx][ny] = true;
            DfsAlgo::dfs(maze, visited, nx, ny)?;
        }
        Ok(())
    }
}

impl algo::MazeAlgo for DfsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        let size = maze.get_size();
        let mut visited = vec![vec![false; size]; size];
        visited[size/2][size/2] = true;
        DfsAlgo::dfs(maze, &mut visited, size/2, size/2)
    }
}
