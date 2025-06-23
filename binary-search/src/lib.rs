pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len().checked_sub(1)?;

    while low <= high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    break; // Prevent underflow
                }
                high = mid - 1;
            }
        }
    }

    None
}
