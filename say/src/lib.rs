pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let scales = [
        "", "thousand", "million", "billion", "trillion",
        "quadrillion", "quintillion",
    ];

    let mut result = Vec::new();
    let mut n = n;
    let mut scale_idx = 0;

    while n > 0 {
        let chunk = (n % 1000) as u16;
        if chunk > 0 {
            let mut chunk_words = say_chunk(chunk);
            if !scales[scale_idx].is_empty() {
                chunk_words.push(scales[scale_idx]);
            }
            result.push(chunk_words.join(" "));
        }
        n /= 1000;
        scale_idx += 1;
    }

    result.reverse();
    result.join(" ")
}

fn say_chunk(n: u16) -> Vec<&'static str> {
    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut parts = Vec::new();
    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
        parts.push(below_20[hundreds as usize]);
        parts.push("hundred");
    }

    if remainder > 0 {
        if remainder < 20 {
            parts.push(below_20[remainder as usize]);
        } else {
            let t = remainder / 10;
            let u = remainder % 10;
            let tens_part = tens[t as usize];
            if u == 0 {
                parts.push(tens_part);
            } else {
                // join hyphenated for things like twenty-two
                let combined = Box::leak(format!("{}-{}", tens_part, below_20[u as usize]).into_boxed_str());
                parts.push(combined);
            }
        }
    }

    parts
}
