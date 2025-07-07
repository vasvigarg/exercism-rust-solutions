pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            // Remove all punctuation except hyphens (already handled in split)
            let cleaned: String = word.chars().filter(|c| c.is_alphabetic()).collect();
            
            if cleaned.is_empty() {
                return Vec::new();
            }
            
            // Check if word is all uppercase (like "GNU")
            if cleaned.chars().all(|c| c.is_ascii_uppercase()) {
                // For all-caps words, just take the first letter
                vec![cleaned.chars().next().unwrap()]
            } else {
                // Handle camelCase: first letter + any subsequent capital letters
                let mut result = Vec::new();
                let mut chars = cleaned.chars().peekable();
                
                if let Some(first_char) = chars.next() {
                    result.push(first_char.to_ascii_uppercase());
                    
                    // Add any subsequent capital letters
                    for ch in chars {
                        if ch.is_ascii_uppercase() {
                            result.push(ch);
                        }
                    }
                }
                
                result
            }
        })
        .collect()
}