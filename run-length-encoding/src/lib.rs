pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut encoded = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 1;

    let mut current = chars.next().unwrap();

    while let Some(&next) = chars.peek() {
        if next == current {
            count += 1;
        } else {
            if count == 1 {
                encoded.push(current);
            } else {
                encoded.push_str(&format!("{}{}", count, current));
            }
            current = next;
            count = 1;
        }
        chars.next();
    }

    // Handle the final group
    if count == 1 {
        encoded.push(current);
    } else {
        encoded.push_str(&format!("{}{}", count, current));
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();

    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count_str.push(ch);
        } else {
            let count = count_str.parse::<usize>().unwrap_or(1);
            decoded.push_str(&ch.to_string().repeat(count));
            count_str.clear();
        }
    }

    decoded
}
