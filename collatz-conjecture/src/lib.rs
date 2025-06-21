pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None; // Collatz is undefined for 0
    }

    let mut steps = 0;
    let mut current = n;

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            // Handle potential overflow for large odd numbers
            match current.checked_mul(3)?.checked_add(1) {
                Some(next) => current = next,
                None => return None,
            }
        }
        steps += 1;
    }

    Some(steps)
}
