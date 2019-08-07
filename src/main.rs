use std::env;
use std::error::Error;
use std::fmt;

mod algos;
use crate::algos::algo::MazeAlgo;
mod maze;

#[derive(Debug)]
struct MazeError;

impl Error for MazeError {
    fn description(&self) -> &str {
        "Error in the maze"
    }
}

impl fmt::Display for MazeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in the maze")
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
    println!("{:?}", args.len());
    if args.len() <= 1 {
        return Err("Oh crap".to_string());
    }
    let mut m = maze::Maze::new(args[1].as_str().parse::<usize>().unwrap());
    m.print();
    algos::dfs::DfsAlgo::generate(&mut m)?;
    m.print();
    Ok(())
}
