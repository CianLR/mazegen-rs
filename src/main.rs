mod algos;
mod maze;

use algos::algo::MazeAlgo;
use maze::Maze;

fn apply_algo(alg: &String, mut maze: &mut Maze) -> Result<(), String> {
    match alg.as_ref() {
        "dfs" => algos::dfs::DfsAlgo::generate(&mut maze),
        "kruskals" => algos::kruskals::KruskalsAlgo::generate(&mut maze),
        "wilsons" => algos::wilsons::WilsonsAlgo::generate(&mut maze),
        "ellers" => algos::ellers::EllersAlgo::generate(&mut maze),
        _ => Err("Algorithm not found".to_string()),
    }
}

fn get_maze(size: &String) -> Result<Maze, String> {
    let sz = match size.as_str().parse::<usize>() {
       Err(_) => Err("Couldn't parse size".to_string()),
       Ok(r) => Ok(r),
    }?;
    Ok(Maze::new(sz))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2 {
        return Err("Usage: mazegen <algorithm> <size>".to_string());
    }
    //let alg = get_algo(&args[1])?;
    let mut m = get_maze(&args[2])?;
    //alg::generate(&mut m);
    apply_algo(&args[1], &mut m)?;
    m.print();
    Ok(())
}
