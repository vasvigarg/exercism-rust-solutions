pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return vec![];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|window| String::from_utf8(window.to_vec()).unwrap())
        .collect()
}
