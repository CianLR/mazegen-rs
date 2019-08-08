use std::time::SystemTime;

use crate::algos::algo;
use crate::maze::{Maze, Walls, wall_opposite};

pub struct DfsAlgo;

// TODO: Get a way better random func
fn rand_num(mx: u64) -> u64 {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let a = (t % 100007) % mx as u128;
    a as u64
}

fn get_adjacent(sz: usize, x: usize, y: usize) -> Vec<(usize, usize, Walls)> {
    let mut v = vec![];
    if x > 0 { v.push((x - 1, y, Walls::Left)); }
    if y > 0 { v.push((x, y - 1, Walls::Up)); }
    if x + 1 < sz { v.push((x + 1, y, Walls::Right)); }
    if y + 1 < sz { v.push((x, y + 1, Walls::Down)); }
    v
}

impl DfsAlgo {
    fn dfs(maze: &mut Maze, visited: &mut Vec<Vec<bool>>,
           sz: usize, x: usize, y: usize) -> Result<(), String> {
        let mut adj: Vec<(usize, usize, Walls)> = get_adjacent(sz, x, y)
            .iter()
            .filter(| (nx, ny, _) | !visited[*nx][*ny])
            .map(| p | *p)
            .collect::<Vec<(usize, usize, Walls)>>();
        while !adj.is_empty() {
            let (nx, ny, d) = adj.remove(rand_num(adj.len() as u64) as usize);
            if visited[nx][ny] {
                continue;
            }
            maze.rm_wall(x, y, d);
            maze.rm_wall(nx, ny, wall_opposite(d));
            visited[nx][ny] = true;
            DfsAlgo::dfs(maze, visited, sz, nx, ny)?;
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
        DfsAlgo::dfs(maze, &mut visited, size, size/2, size/2)
    }
}
