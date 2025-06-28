pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();

    if len == 0 {
        return String::new();
    }

    let mut lines = Vec::new();

    for i in 0..len - 1 {
        lines.push(format!(
            "For want of a {} the {} was lost.",
            list[i],
            list[i + 1]
        ));
    }

    lines.push(format!(
        "And all for the want of a {}.",
        list[0]
    ));

    lines.join("\n")
}
