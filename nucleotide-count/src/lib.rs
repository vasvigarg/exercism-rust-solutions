use std::collections::HashMap;

/// Returns the count of a specific nucleotide in a DNA string.
/// Returns `Err(invalid_char)` if either the nucleotide or any character in the DNA string is invalid.
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !matches!(nucleotide, 'A' | 'C' | 'G' | 'T') {
        return Err(nucleotide);
    }

    for ch in dna.chars() {
        if !matches!(ch, 'A' | 'C' | 'G' | 'T') {
            return Err(ch);
        }
    }

    Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
}

/// Returns a count of each valid nucleotide in the given DNA string.
/// Returns `Err(invalid_char)` if any invalid character is found.
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    for ch in dna.chars() {
        if let Some(counter) = counts.get_mut(&ch) {
            *counter += 1;
        } else {
            return Err(ch);
        }
    }

    Ok(counts)
}
