pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            result.push(divisor);
            n /= divisor;
        }
        divisor += 1;

        if divisor * divisor > n && n > 1 {
            result.push(n);
            break;
        }
    }

    result
}
