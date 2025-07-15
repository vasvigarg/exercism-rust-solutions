use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score_map = HashMap::new();

    // Assign points to each group of letters
    score_map.extend("AEIOULNRST".chars().map(|c| (c, 1)));
    score_map.extend("DG".chars().map(|c| (c, 2)));
    score_map.extend("BCMP".chars().map(|c| (c, 3)));
    score_map.extend("FHVWY".chars().map(|c| (c, 4)));
    score_map.insert('K', 5);
    score_map.extend("JX".chars().map(|c| (c, 8)));
    score_map.extend("QZ".chars().map(|c| (c, 10)));

    // Compute the score
    word.chars()
        .map(|c| c.to_ascii_uppercase())
        .filter_map(|c| score_map.get(&c))
        .copied()
        .sum()
}
