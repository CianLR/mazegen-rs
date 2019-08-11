use rand::prelude::*;

use crate::algos::algo::MazeAlgo;
use crate::maze::Maze;

#[derive(Clone, PartialEq)]
enum CellInfo {
    Blank,
    CurPath,
    InMaze,
}

pub struct WilsonsAlgo {
    animate: bool
}

impl WilsonsAlgo {
    pub fn new(animate: bool) -> WilsonsAlgo {
        WilsonsAlgo { animate: animate }
    }

    fn wilsons(&self, mut maze: &mut Maze) -> Result<(), String> {
        let size = maze.get_size();
        let mut cells = vec![vec![CellInfo::Blank; size]; size];
        cells[0][0] = CellInfo::InMaze;
        for x in 0..size {
            for y in 0..size {
                if x == 1 && y == 1 {
                    continue;
                }
                if self.animate { self.frame(maze, 20); }
                let a = WilsonsAlgo::walk(&mut maze, &mut cells, x, y, x, y);
                if a.is_some() {
                    // This should be impossible... I think.
                    return Err("Paths exhausted".to_string());
                }
            }
        }
        Ok(())
    }

    fn walk(mut maze: &mut Maze, mut cells: &mut Vec<Vec<CellInfo>>,
            x: usize, y: usize, px: usize, py: usize) -> Option<(usize, usize)> {
        if cells[x][y] == CellInfo::InMaze {
            return None;
        } else if cells[x][y] == CellInfo::CurPath {
            return Some((x, y));
        }
        cells[x][y] = CellInfo::CurPath;
        let mut adj = maze.get_adjacent(x, y);
        adj.shuffle(&mut rand::thread_rng());
        while !adj.is_empty() {
            let (x2, y2) = adj.pop().unwrap();
            if x2 == px && y2 == py {
                continue;  //  Don't move backwards.
            }
            let res = WilsonsAlgo::walk(&mut maze, &mut cells, x2, y2, x, y);
            if res.is_none() {
                cells[x][y] = CellInfo::InMaze;
                maze.remove_wall(x, y, x2, y2);
                return None;
            }
            // We're erasing the loop.
            let (lx, ly) = res.unwrap();
            if lx != x || ly != y {
                cells[x][y] = CellInfo::Blank;
                return Some((lx, ly));
            }
        }
        // All paths exhausted.
        cells[x][y] = CellInfo::Blank;
        Some((px, py))
    }
}

impl MazeAlgo for WilsonsAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String> {
        self.wilsons(maze)
    }
}
