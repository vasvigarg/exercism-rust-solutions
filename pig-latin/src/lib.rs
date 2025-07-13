
pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    
    // Rule 1: Word begins with a vowel, "xr", or "yt"
    if starts_with_vowel(&chars) || starts_with_xr(&chars) || starts_with_yt(&chars) {
        return format!("{}ay", word);
    }
    
    // Find the position where we should split the word
    let split_pos = find_split_position(&chars);
    
    // Split the word and rearrange
    let (prefix, suffix) = word.split_at(split_pos);
    format!("{}{}ay", suffix, prefix)
}

fn starts_with_vowel(chars: &[char]) -> bool {
    if chars.is_empty() {
        return false;
    }
    matches!(chars[0], 'a' | 'e' | 'i' | 'o' | 'u')
}

fn starts_with_xr(chars: &[char]) -> bool {
    chars.len() >= 2 && chars[0] == 'x' && chars[1] == 'r'
}

fn starts_with_yt(chars: &[char]) -> bool {
    chars.len() >= 2 && chars[0] == 'y' && chars[1] == 't'
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn find_split_position(chars: &[char]) -> usize {
    let mut i = 0;
    
    // Skip consonants until we find a vowel or special case
    while i < chars.len() {
        let current = chars[i];
        
        // Rule 3: Handle "qu" pattern
        if current == 'q' && i + 1 < chars.len() && chars[i + 1] == 'u' {
            return i + 2; // Move past "qu"
        }
        
        // Rule 4: Handle "y" as vowel when preceded by consonants
        if current == 'y' && i > 0 {
            return i; // Split before 'y'
        }
        
        // If we hit a vowel (not 'y'), split here
        if is_vowel(current) {
            return i;
        }
        
        i += 1;
    }
    
    // If we reach here, the word is all consonants (shouldn't happen with valid input)
    chars.len()
}