use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let normalized_target = normalize(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            word.to_lowercase() != candidate.to_lowercase()
                && normalize(candidate) == normalized_target
        })
        .collect()
}

fn normalize(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}
