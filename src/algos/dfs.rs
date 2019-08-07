use std::time::SystemTime;

use crate::algos::algo;
use crate::maze::{Maze, Walls, wall_opposite};

pub struct DfsAlgo;

fn rand_num(mx: u64) -> u64 {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    t % mx
}

fn random_order<T>(v: &mut Vec<T>) -> Vec<T> {
    let mut nv = vec![];
    while !v.is_empty() {
        nv.push(v.remove(rand_num(v.len() as u64) as usize));
    }
    nv
}

impl algo::MazeAlgo for DfsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        let size = maze.get_size();
        let mut visited = vec![vec![false; size]; size];
        visited[0][0] = true;
        let mut stack = vec![(0_usize, 0_usize, 0_usize, 0_usize, Walls::Up)];
        while !stack.is_empty() {
            let (x, y, xl, yl, w) = stack.pop().unwrap();
            println!("At coords {}, {}", x, y);
            // Mark walls
            if x != xl || y != yl {
                maze.rm_wall(xl, yl, w);
                maze.rm_wall(x, y, wall_opposite(w));
            }
            // Gather neighbours
            let mut neighbours = vec![];
            if x > 0 && !visited[x - 1][y] {
                neighbours.push((x - 1, y, x, y, Walls::Left));
                visited[x - 1][y] = true;
            }
            if y > 0 && !visited[x][y - 1] {
                neighbours.push((x, y - 1, x, y, Walls::Up));
                visited[x][y - 1] = true;
            }
            if x + 1 < size && !visited[x + 1][y] {
                neighbours.push((x + 1, y, x, y, Walls::Right));
                visited[x + 1][y] = true;
            }
            if y + 1 < size && !visited[x][y + 1] {
                neighbours.push((x, y + 1, x, y, Walls::Down));
                visited[x][y + 1] = true;
            }
            // TODO(CianLR): Randomise order.
            //stack.extend(random_order(&mut neighbours))
            stack.extend(neighbours);
            stack = random_order(&mut stack);
        }
        Ok(())
    }
}

