use crate::maze::Maze;

pub trait MazeAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String>;
}

