pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=((upper_bound as f64).sqrt() as u64) {
        if is_prime[num as usize] {
            let mut multiple = num * num;
            while multiple <= upper_bound {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p { Some(i as u64) } else { None })
        .collect()
}
