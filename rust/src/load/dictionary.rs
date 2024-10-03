use std::fs;
use std::io;

pub fn load_dictionary(filename: &str) -> Result<Vec<String>, io::Error> {
    // Read the file contents into a string
    let contents = fs::read_to_string(filename)?;

    let words: Vec<String> = contents
        .split_whitespace()
        .filter(|s| s.len() > 2)
        .map(|s| s.to_string())
        .collect();

    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dictionary() {
        let result = load_dictionary("../test_data/dict_valid.txt");
        assert!(result.is_ok());

        if let Ok(dict) = result {
            assert_eq!(dict.len(), 2);
        }
    }
}
