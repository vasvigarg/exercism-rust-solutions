pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if input.is_empty() || input[0].is_empty() {
        return result;
    }

    let row_count = input.len();
    let col_count = input[0].len();

    // Compute max values in each row
    let row_maxes: Vec<u64> = input
        .iter()
        .map(|row| *row.iter().max().unwrap())
        .collect();

    // Compute min values in each column
    let mut col_mins = vec![u64::MAX; col_count];
    for col in 0..col_count {
        for row in 0..row_count {
            col_mins[col] = col_mins[col].min(input[row][col]);
        }
    }

    // Find all saddle points
    for row in 0..row_count {
        for col in 0..col_count {
            let value = input[row][col];
            if value == row_maxes[row] && value == col_mins[col] {
                result.push((row, col));
            }
        }
    }

    result
}
