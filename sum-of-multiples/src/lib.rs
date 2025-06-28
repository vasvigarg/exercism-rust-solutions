use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new();

    for &factor in factors {
        if factor == 0 {
            continue; // Skip zero to avoid infinite loop / division by zero
        }

        let mut multiple = factor;
        while multiple < limit {
            unique_multiples.insert(multiple);
            multiple += factor;
        }
    }

    unique_multiples.iter().sum()
}
