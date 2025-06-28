/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned: String = code.chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    if cleaned.len() <= 1 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = cleaned
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                digit
            }
        })
        .sum();

    sum % 10 == 0
}
