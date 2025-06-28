pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return vec![];
    }
    let width = minefield[0].len();
    if width == 0 {
        return vec![String::new(); height];
    }

    let mut result = Vec::with_capacity(height);

    for (row, &line) in minefield.iter().enumerate() {
        let mut row_result = String::with_capacity(width);
        let bytes = line.as_bytes();

        for col in 0..width {
            if bytes[col] == b'*' {
                row_result.push('*');
                continue;
            }

            let mut count = 0;
            for dr in [-1i32, 0, 1] {
                for dc in [-1i32, 0, 1] {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;
                    if nr >= 0
                        && nc >= 0
                        && (nr as usize) < height
                        && (nc as usize) < width
                        && minefield[nr as usize].as_bytes()[nc as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                row_result.push(' ');
            } else {
                row_result.push((b'0' + count) as char);
            }
        }

        result.push(row_result);
    }

    result
}
