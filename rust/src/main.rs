use rust_boggle::load::board::Board;
use rust_boggle::load::dictionary::load_dictionary;
use rust_boggle::search_tree::trie::Trie;
use rust_boggle::solver::boggle_trie::traverse_board;
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
    solve(filename.clone());
}

fn solve(filename: String) {
    // load the board
    let board = match Board::load(&filename) {
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
    let dict = match load_dictionary("../words.txt") {
        Ok(d) => {
            println!("Dictionary loaded successfully!");
            d
        }
        Err(e) => {
            eprintln!("Failed to load dictionary: {}", e);
            process::exit(1)
        }
    };

    // solve
    let mut trie = Trie::from_dict(dict);
    let found_words = traverse_board(&board, &mut trie);

    // print solutions

    // debug
    println!("solutions: {:?}", &found_words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_by_three() {
        solve("../test_data/threeByThree.txt".to_string());
    }

    #[test]
    fn test_ten_by_ten() {
        solve("../test_data/tenByTen.txt".to_string());
    }
}
