pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut n = display_value;

    while n > 0 {
        if n % 2 == 1 {
            count += 1;
        }
        n /= 2;
    }

    count
}
