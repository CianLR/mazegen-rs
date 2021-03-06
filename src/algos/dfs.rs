use rand::prelude::*;

use crate::algos::algo::MazeAlgo;
use crate::maze::Maze;

pub struct DfsAlgo {
    animate: bool
}

impl DfsAlgo {
    pub fn new(animate: bool) -> DfsAlgo {
        DfsAlgo { animate: animate }
    }

    fn dfs(&self, maze: &mut Maze, visited: &mut Vec<Vec<bool>>,
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
            if self.animate {
                self.frame(maze, 20);
            }
            maze.remove_wall(x, y, nx, ny);
            visited[nx][ny] = true;
            self.dfs(maze, visited, nx, ny)?;
        }
        Ok(())
    }
}

impl MazeAlgo for DfsAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String> {
        let size = maze.get_size();
        let mut visited = vec![vec![false; size]; size];
        visited[size/2][size/2] = true;
        self.dfs(maze, &mut visited, size/2, size/2)
    }
}


#[cfg(test)]
mod test {
    use crate::algos::test_util::*;

    #[test]
    fn test_is_perfect() {
        let m = apply_test_algo("dfs");
        assert!(is_perfect_maze(&m));
    }
}
