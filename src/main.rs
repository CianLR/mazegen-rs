#[macro_use]
extern crate clap;

mod algos;
mod maze;

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
            .possible_values(&algos::ALGORITHMS)
            .help("Sets the algorithm used to generate the maze"))
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .index(2)
            .required(true)
            .takes_value(true)
            .help("Sets the size of the maze to be generated"))
        .arg(Arg::with_name("animate")
            .long("animate")
            .help("Draw the process of creating the maze on screen"))
        .get_matches()
}

fn main() -> Result<(), String> {
    let args = get_args();
    let algo = args.value_of("algorithm").unwrap().to_string();
    let size = value_t!(args, "size", usize).unwrap_or_else(|e| e.exit());

    let mut m = Maze::new(size);
    m.apply_algorithm(&mut algos::get_algorithm(&algo)?)?;
    m.print();
    Ok(())
}
