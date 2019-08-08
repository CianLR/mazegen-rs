use rand::prelude::*;

use crate::algos::algo;
use crate::maze::{Maze, Walls};

pub struct DfsAlgo;

impl DfsAlgo {
    fn dfs(maze: &mut Maze, visited: &mut Vec<Vec<bool>>,
           sz: usize, x: usize, y: usize) -> Result<(), String> {
        let mut adj: Vec<(usize, usize, Walls)> = DfsAlgo::get_adjacent(sz, x, y)
            .into_iter()
            .filter(|&(nx, ny, _)| !visited[nx][ny])
            .collect::<Vec<(usize, usize, Walls)>>();
        adj.shuffle(&mut rand::thread_rng());
        while !adj.is_empty() {
            let (nx, ny, d) = adj.pop().unwrap();
            if visited[nx][ny] {
                continue;
            }
            maze.rm_wall(x, y, d);
            maze.rm_wall(nx, ny, d.opposite());
            visited[nx][ny] = true;
            DfsAlgo::dfs(maze, visited, sz, nx, ny)?;
        }
        Ok(())
    }

    fn get_adjacent(sz: usize, x: usize, y: usize) -> Vec<(usize, usize, Walls)> {
        let mut v = vec![];
        if x > 0 { v.push((x - 1, y, Walls::Left)); }
        if y > 0 { v.push((x, y - 1, Walls::Up)); }
        if x + 1 < sz { v.push((x + 1, y, Walls::Right)); }
        if y + 1 < sz { v.push((x, y + 1, Walls::Down)); }
        v
    }
}

impl algo::MazeAlgo for DfsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        let size = maze.get_size();
        let mut visited = vec![vec![false; size]; size];
        visited[size/2][size/2] = true;
        DfsAlgo::dfs(maze, &mut visited, size, size/2, size/2)
    }
}
