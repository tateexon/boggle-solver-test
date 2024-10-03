use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum BoardError {
    IoError(io::Error),
    InvalidBoard(String),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::IoError(e) => write!(f, "IO error: {}", e),
            BoardError::InvalidBoard(msg) => write!(f, "Invalid board: {}", msg),
        }
    }
}

impl std::error::Error for BoardError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BoardError::IoError(e) => Some(e),
            BoardError::InvalidBoard(_) => None,
        }
    }
}

// Implement `From` to convert `io::Error` into `BoardError`
impl From<io::Error> for BoardError {
    fn from(error: io::Error) -> Self {
        BoardError::IoError(error)
    }
}

#[derive(Debug)]
pub struct Board {
    pub values: Vec<char>,
    pub size: i32,
}

impl Board {
    pub fn load(filename: &str) -> Result<Self, BoardError> {
        // Read the file contents into a string
        let contents = fs::read_to_string(filename)?;

        // Process the contents to get the board values
        let values = get_clean_board(contents);

        // Calculate the board size
        let size = calc_board_size(&values);

        let size_squared = size * size;
        let len = values.len();
        if size_squared != len as i32 {
            eprintln!(
                "Board Size: {}, likely wanted size: {}x{}",
                len,
                size_squared + 1,
                size_squared + 1
            );
            return Err(BoardError::InvalidBoard(
                "Board is not a perfect square. Add, remove, or fix values to create a valid board.".to_string(),
            ));
        }

        // Construct the board
        let board = Board { values, size };

        Ok(board)
    }
}

fn get_clean_board(contents: String) -> Vec<char> {
    let mut chars = Vec::new();

    // Split the contents by whitespace and collect into a vector of Strings
    let words: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    for word in words {
        let mut char_iter = word.chars();
        if let Some(c) = char_iter.next() {
            let u = char_iter.next();
            if u.is_none() || (c == 'q' && u.unwrap() == 'u' && char_iter.next().is_none()) {
                chars.push(c)
            } else {
                println!("Invalid character in board: {}", word)
            }
        }
    }
    chars
}

fn calc_board_size(values: &[char]) -> i32 {
    (values.len() as f32).sqrt() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_clean_board() {
        let letters: String = String::from("a b c d qu q d e fff f");
        let board: Vec<char> = get_clean_board(letters);
        assert_eq!(board.len(), 9);
        assert_eq!(board[0], 'a');
        assert_eq!(board[4], 'q');
        assert_eq!(board[8], 'f');
    }

    #[test]
    fn test_calc_board_values() {
        let board: Vec<char> = vec!['a', 'b', 'c', 'd'];
        let size: i32 = calc_board_size(&board);
        assert_eq!(size, 2)
    }

    // add tests to load test boards from parent directory
    #[test]
    fn test_board_valid() {
        let result = Board::load("../test_boards/threeByThree.txt");
        assert!(
            result.is_ok(),
            "Should have had a valid board load: {:?}",
            result
        );

        if let Ok(board) = result {
            assert_eq!(board.values.len(), 9);
            assert_eq!(board.values[0], 'a');
            assert_eq!(board.values[8], 'j');
        }
    }

    #[test]
    fn test_board_invalid() {
        let result = Board::load("../test_boards/invalid.txt");
        assert!(
            result.is_err(),
            "Expected the board load to be an error, got: {:?}",
            result
        );
    }
}
