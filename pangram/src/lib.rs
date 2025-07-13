use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();

    for c in sentence.chars() {
        if c.is_ascii_alphabetic() {
            letters.insert(c.to_ascii_lowercase());
        }
    }

    letters.len() == 26
}
