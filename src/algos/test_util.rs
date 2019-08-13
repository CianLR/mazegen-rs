use crate::algos::get_algorithm;
use crate::maze::Maze;

pub fn apply_test_algo(algo: &str) -> Maze {
    let size = 50;
    let mut m = Maze::new(size);
    let mut algo = get_algorithm(&algo.to_string(), false).unwrap();
    m.apply_algorithm(&mut algo).unwrap();
    m
}

pub fn is_perfect_maze(maze: &Maze) -> bool {
    let size = maze.get_size();
    let mut seen = vec![vec![false; size]; size];
    let mut stack = vec![(0, 0, 0, 0)];
    seen[0][0] = true;
    while !stack.is_empty() {
       let (x, y, px, py) = stack.pop().unwrap();
       for (x2, y2) in maze.get_open_adj(x, y) {
           if !seen[x2][y2] {
               seen[x2][y2] = true;
               stack.push((x2, y2, x, y));
           } else if x2 != px || y2 != py {
               // If the seen cell is not the parent then it's a loop.
               return false;
           }
       }
    }
    // Return true iif everything in seen is true.
    seen.into_iter().flatten().fold(true, |a, b| a && b)
}

