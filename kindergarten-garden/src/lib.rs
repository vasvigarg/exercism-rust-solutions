const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
    "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
];

fn plant_name(ch: char) -> &'static str {
    match ch {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "unknown",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut rows = diagram.lines();
    let row1 = rows.next().unwrap_or("");
    let row2 = rows.next().unwrap_or("");

    let mut sorted = STUDENTS.to_vec();
    sorted.sort();
    let index = sorted.iter().position(|&s| s == student).unwrap();
    let start = index * 2;

    let chars1: Vec<char> = row1.chars().collect();
    let chars2: Vec<char> = row2.chars().collect();

    vec![
        plant_name(chars1[start]),
        plant_name(chars1[start + 1]),
        plant_name(chars2[start]),
        plant_name(chars2[start + 1]),
    ]
}
