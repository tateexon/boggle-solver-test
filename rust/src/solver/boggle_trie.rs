use std::collections::HashSet;

use crate::{
    load::board::Board,
    search_tree::trie::{FindWordResult, Trie},
    vec::flat_vec_2d::{get_2d_coordinates, get_2d_index},
};

const ADJACENT_TILES: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

pub fn find_words(
    board: &Board,
    current_position: i32,
    visited_positions: &mut Vec<i32>,
    current_partial_word: &mut String,
    trie_node: &mut Trie,
    depth: i16,
) -> HashSet<String> {
    let mut found_words: HashSet<String> = HashSet::new();
    if depth > 15 {
        return found_words;
    }

    let (result, _) = trie_node.find_word(current_partial_word);

    // back out early if char not found
    if result == FindWordResult::None {
        current_partial_word.pop();
        return found_words;
    }

    match result {
        FindWordResult::Found => {
            found_words.insert(current_partial_word.to_string().replace("q", "qu"))
        }
        FindWordResult::None => {
            current_partial_word.pop();
            return found_words;
        }
        FindWordResult::Partial => {
            // println!("nothing to see here");
            true
        }
    };

    for (next_x, next_y) in ADJACENT_TILES.iter() {
        let (x, y) = get_2d_coordinates(current_position, board.size);
        let new_x = x + next_x;
        let new_y = y + next_y;
        let index = (current_position + get_2d_index(new_x, new_y, board.size)) as usize;

        if index as i32 >= board.size * board.size || (index as i32) < 0 {
            // println!("how did we get here?");
            break;
        } else {
            // add the character to the string
            if let Some(c) = board.values.get(index) {
                current_partial_word.push(*c);
            }

            let last_visited_index = visited_positions.len();
            visited_positions.push(index as i32);

            let words_found_within = find_words(
                board,
                index as i32,
                visited_positions,
                current_partial_word,
                trie_node,
                depth + 1,
            );
            found_words.extend(words_found_within);

            visited_positions.remove(last_visited_index);
        }
    }

    current_partial_word.pop();

    found_words
}

pub fn traverse_board(board: &Board, trie_node: &mut Trie) -> HashSet<String> {
    let mut words: HashSet<String> = HashSet::new();
    let mut visited_positions;
    let mut current_partial_word;

    for (i, c) in board.values.iter().enumerate() {
        current_partial_word = c.to_string();
        visited_positions = vec![i as i32];

        let found_words_for_char = find_words(
            board,
            i as i32,
            &mut visited_positions,
            &mut current_partial_word,
            trie_node,
            1,
        );
        words.extend(found_words_for_char);
    }

    words
}
