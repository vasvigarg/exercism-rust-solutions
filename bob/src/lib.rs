pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if trimmed.ends_with('?') && trimmed.chars().any(|c| c.is_alphabetic()) && trimmed == trimmed.to_uppercase() {
        "Calm down, I know what I'm doing!"
    } else if trimmed.chars().any(|c| c.is_alphabetic()) && trimmed == trimmed.to_uppercase() {
        "Whoa, chill out!"
    } else if trimmed.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
