use rust_boggle::load::board::Board;
use std::env;
use std::process;

fn main() {
    // validate arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    // load the board
    let board = match Board::load(filename) {
        Ok(b) => {
            println!("Board loaded successfully!");
            b
        }
        Err(e) => {
            eprintln!("Failed to load board: {}", e);
            process::exit(1)
        }
    };

    // load the dictionary

    // solve

    // print solutions

    // debug
    println!("{:?}", &board.values);
}
