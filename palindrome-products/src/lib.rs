use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    fn new(value: u64, factors: HashSet<(u64, u64)>) -> Self {
        Palindrome { value, factors }
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut pal_map: std::collections::BTreeMap<u64, HashSet<(u64, u64)>> = std::collections::BTreeMap::new();

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                let entry = pal_map.entry(product).or_insert_with(HashSet::new);
                entry.insert((i, j));
            }
        }
    }

    if pal_map.is_empty() {
        return None;
    }

    let (min_val, min_factors) = pal_map.iter().next().unwrap();
    let (max_val, max_factors) = pal_map.iter().next_back().unwrap();

    Some((
        Palindrome::new(*min_val, min_factors.clone()),
        Palindrome::new(*max_val, max_factors.clone()),
    ))
}
