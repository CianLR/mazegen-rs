use std::collections::HashSet;
use std::cmp::max;

use rand::prelude::*;

use crate::algos::algo::MazeAlgo;
use crate::maze::Maze;


pub struct EllersAlgo {
    animate: bool
}

impl EllersAlgo {
    pub fn new(animate: bool) -> EllersAlgo {
        EllersAlgo { animate: animate }
    }

    fn ellers(&self, mut maze: &mut Maze) -> Result<(), String> {
        let size = maze.get_size();
        // This vector holds the current row of the maze.
        let mut curr = (1..(size + 1)).collect();
        for y in 0..(size - 1) {
            self.join_adjacent(&mut maze, &mut curr, y);
            let prev = curr;
            curr = vec![0; size];
            self.make_verticals(&mut maze, &prev, &mut curr, y);
        }
        self.join_last(&mut maze, &mut curr);
        Ok(())
    }

    fn join_last(&self, maze: &mut Maze, curr: &mut Vec<usize>) {
        self.assign_sets(curr);
        let size = maze.get_size();
        for x in 0..(size - 1) {
            if curr[x] != curr[x + 1] {
                self.vector_replace(curr, curr[x + 1], curr[x]);
                if self.animate { self.frame(maze, 20); }
                maze.remove_wall(x, size - 1, x + 1, size - 1);
            }
        }
    }

    fn join_adjacent(&self,
                     maze: &mut Maze,
                     curr: &mut Vec<usize>,
                     y: usize) {
        let mut rng = rand::thread_rng();
        self.assign_sets(curr);
        for x in 0..(maze.get_size() - 1) {
            if curr[x] != curr[x + 1] {
                // Maybe join (2 / 3 chance).
                if rng.gen_range(0, 2) > 0 {
                    self.vector_replace(curr, curr[x + 1], curr[x]);
                    if self.animate { self.frame(maze, 20); }
                    maze.remove_wall(x, y, x + 1, y);
                }
            }
        }
    }

    fn assign_sets(&self, curr: &mut Vec<usize>) {
        // Get max of list, definitely some better way
        // TODO(CianLR): Find that way.
        let mut next_set = curr.iter().fold(0_usize, |a, b| max(a, *b)) + 1;
        for x in 0..curr.len() {
            if curr[x] == 0 {
                curr[x] = next_set;
                next_set += 1;
            }
        }
    }

    fn vector_replace(&self, v: &mut Vec<usize>, old: usize, new: usize) {
        for i in 0..v.len() {
            if v[i] == old {
                v[i] = new;
            }
        }
    }

    fn make_verticals(&self,
                      maze: &mut Maze,
                      prev: &Vec<usize>,
                      curr: &mut Vec<usize>,
                      y: usize) {
        let size = maze.get_size();
        let mut rng = rand::thread_rng();
        let mut seen = HashSet::new();
        for x in 0..size {
            if seen.contains(&prev[x]) {
                continue;
            }
            seen.insert(prev[x]);
            // Gather the members of this set
            let mut indices = vec![x];
            for x2 in (x + 1)..size {
                if prev[x2] == prev[x] {
                    indices.push(x2);
                }
            }
            // For each set randomly choose between 1 and size(set) verticals.
            indices.shuffle(&mut rng);
            let ver_count = rng.gen_range(1, indices.len() + 1);
            for v in 0..ver_count {
                curr[indices[v]] = prev[x];
                if self.animate { self.frame(maze, 20); }
                maze.remove_wall(indices[v], y, indices[v], y + 1);
            }
        }
    }
}

impl MazeAlgo for EllersAlgo {
    fn generate(&mut self, maze: &mut Maze) -> Result<(), String> {
        self.ellers(maze)
    }
}


#[cfg(test)]
mod test {
    use crate::algos::test_util::*;

    #[test]
    fn test_is_perfect() {
        let m = apply_test_algo("ellers");
        assert!(is_perfect_maze(&m));
    }
}
