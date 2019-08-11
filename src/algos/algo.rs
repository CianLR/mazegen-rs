use crate::maze::Maze;

pub trait MazeAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String>;
}

pub static ALGORITHMS: [&str; 5] = [
    "dfs",
    "kruskals",
    "wilsons",
    "ellers",
    "prims",
];

