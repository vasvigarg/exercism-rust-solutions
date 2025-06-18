pub fn recite(start: u32, take: u32) -> String {
    (0..take)
        .map(|i| verse(start - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let bottle_word = |count| if count == 1 { "bottle" } else { "bottles" };
    let curr = capitalize(num_to_words(n));
    let next_n = n.saturating_sub(1);
    let next = num_to_words(next_n);

    format!(
        "{0} green {1} hanging on the wall,\n\
         {0} green {1} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {2} green {3} hanging on the wall.",
        curr,
        bottle_word(n),
        next,
        bottle_word(next_n),
    )
}

fn num_to_words(n: u32) -> &'static str {
    match n {
        10 => "ten",
        9 => "nine",
        8 => "eight",
        7 => "seven",
        6 => "six",
        5 => "five",
        4 => "four",
        3 => "three",
        2 => "two",
        1 => "one",
        0 => "no",
        _ => panic!("Unsupported number"),
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}