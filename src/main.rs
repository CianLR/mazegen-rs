#[macro_use]
extern crate clap;

mod algos;
mod maze;

use algos::algo::MazeAlgo;
use maze::Maze;
use clap::{Arg, App};

fn get_args() -> clap::ArgMatches<'static> {
    let version = format!("{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
    App::new("MazeGen")
        .version(&*version)
        .author("CianLR <cian.ruane1@gmail.com>")
        .about("Generates perfect mazes using a variety of algoritms")
        .arg(Arg::with_name("algorithm")
            .short("a")
            .long("algorithm")
            .index(1)
            .required(true)
            .takes_value(true)
            .possible_values(&algos::algo::ALGORITHMS)
            .help("Sets the algorithm used to generate the maze"))
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .index(2)
            .required(true)
            .takes_value(true)
            .help("Sets the size of the maze to be generated"))
        .get_matches()
}

fn apply_algo(alg: &String, mut maze: &mut Maze) -> Result<(), String> {
    match alg.as_ref() {
        "dfs" => algos::dfs::DfsAlgo::generate(&mut maze),
        "kruskals" => algos::kruskals::KruskalsAlgo::generate(&mut maze),
        "wilsons" => algos::wilsons::WilsonsAlgo::generate(&mut maze),
        "ellers" => algos::ellers::EllersAlgo::generate(&mut maze),
        "prims" => algos::prims::PrimsAlgo::generate(&mut maze),
        _ => Err("Algorithm not found".to_string()),
    }
}

fn main() -> Result<(), String> {
    let args = get_args();
    let algo = args.value_of("algorithm").unwrap().to_string();
    let size = value_t!(args, "size", usize).unwrap_or_else(|e| e.exit());

    let mut m = Maze::new(size);
    apply_algo(&algo, &mut m)?;
    m.print();
    Ok(())
}
