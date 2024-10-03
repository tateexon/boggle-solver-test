use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum FindWordResult {
    None,
    Partial,
    Found,
}

#[derive(Debug, Clone)]
pub struct Trie {
    is_word: bool,
    val: Option<char>,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new(c: char, is_word: bool) -> Trie {
        Trie {
            is_word,
            val: Some(c),
            children: HashMap::new(),
        }
    }

    pub fn new_root() -> Trie {
        Trie {
            is_word: false,
            val: None,
            children: HashMap::new(),
        }
    }

    pub fn check(self, c: char) -> bool {
        self.val == Some(c)
    }

    pub fn from_dict(dictionary: Vec<String>) -> Trie {
        let mut root = Trie::new_root();

        for word in dictionary.iter() {
            root.insert_word(word);
        }

        root
    }

    fn insert_word(&mut self, word: &str) {
        let mut current_node: &mut Trie = self;
        let chars: Vec<char> = word.chars().collect();

        for c in &chars {
            // Use entry to insert if not found
            current_node = current_node
                .children
                .entry(*c)
                .or_insert_with(|| Trie::new(*c, false)); // Create a new Trie node if one doesn't exist
        }

        // Mark the last node as the end of a word
        current_node.is_word = true;
    }

    pub fn find_word(&mut self, value: &str) -> (FindWordResult, Option<&mut Trie>) {
        let chars: Vec<char> = value.chars().collect();
        let mut current_node = self;

        for c in &chars {
            if !current_node.children.contains_key(c) {
                return (FindWordResult::None, None);
            }

            current_node = current_node.children.get_mut(c).unwrap();
        }

        if current_node.is_word {
            return (FindWordResult::Found, Some(current_node));
        }

        (FindWordResult::Partial, Some(current_node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dict() {
        let mut t = Trie::from_dict(vec!["hello".to_string(), "any".to_string()]);
        assert_eq!(t.find_word("any").0, FindWordResult::Found);
        assert_eq!(t.find_word("blarg").0, FindWordResult::None);
        let partial = t.find_word("hell");
        assert_eq!(partial.0, FindWordResult::Partial);
        if let Some(p) = partial.1 {
            assert_eq!(p.val.unwrap(), 'l');
            assert_eq!(p.find_word("o").0, FindWordResult::Found);
            assert_eq!(p.find_word("p").0, FindWordResult::None);
        }
    }
}
