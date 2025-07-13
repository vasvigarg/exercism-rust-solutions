use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    for ch in candidate.chars() {
        if ch == ' ' || ch == '-' {
            continue;
        }
        let ch_lower = ch.to_ascii_lowercase();
        if seen.contains(&ch_lower) {
            return false;
        }
        seen.insert(ch_lower);
    }
    true
}
