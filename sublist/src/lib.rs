#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    if first_list == second_list {
        return Equal;
    }

    if is_sublist(first_list, second_list) {
        return Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Superlist;
    }

    Unequal
}

fn is_sublist(small: &[i32], large: &[i32]) -> bool {
    if small.is_empty() {
        return true;
    }

    large
        .windows(small.len())
        .any(|window| window == small)
}
