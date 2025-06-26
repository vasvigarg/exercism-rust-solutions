/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<char> = isbn.chars()
        .filter(|c| *c != '-') 
        .collect();

    if digits.len() != 10 {
        return false;
    }

    let mut sum = 0;

    for (i, ch) in digits.iter().enumerate() {
        let value = if i == 9 && *ch == 'X' {
            10
        } else if ch.is_ascii_digit() {
            ch.to_digit(10).unwrap()
        } else {
            return false; 
        };

        let weight = 10 - i as u32;
        sum += value * weight;
    }

    sum % 11 == 0
}
