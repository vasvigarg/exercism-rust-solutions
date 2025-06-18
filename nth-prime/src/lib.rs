pub fn nth(n: u32) -> u32 {
    let mut primes = vec![];
    let mut candidate = 2;

    while primes.len() <= n as usize {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes[n as usize]
}

fn is_prime(num: u32, primes: &[u32]) -> bool {
    let sqrt = (num as f64).sqrt() as u32;
    for &prime in primes {
        if prime > sqrt {
            break;
        }
        if num % prime == 0 {
            return false;
        }
    }
    true
}
